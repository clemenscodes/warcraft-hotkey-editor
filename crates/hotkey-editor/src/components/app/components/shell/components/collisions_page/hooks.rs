use super::components::body::components::clear_state::ClearStateProps;
use super::components::body::components::details::hotkey_unit_detail::HotkeyUnitDetailProps;
use super::components::body::components::details::island_detail::IslandDetailProps;
use super::components::body::components::details::unit_position_detail::UnitPositionDetailProps;
use super::components::body::components::empty_state::EmptyStateProps;
use super::components::body::components::sidebars::hotkey_unit_sidebar::HotkeyUnitSidebarProps;
use super::components::body::components::sidebars::island_sidebar::IslandSidebarProps;
use super::components::body::components::sidebars::unit_position_sidebar::UnitPositionSidebarProps;
use super::components::body::{ContentModel, HotkeysPane, PositionsPane, UnitPositionsPane};
use super::components::breadcrumbs::BreadcrumbsProps;
use super::logic::{CollisionPageModel, HotkeyCollisionPageModel, UnitPositionPageModel};
use super::props::CollisionsPageProps;
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::grid_layout::use_grid_layout;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::context::{use_synced_route, use_view_navigation};
use crate::services::navigation::nav_snapshot::NavSnapshot;
use dioxus::prelude::*;

/// The shaped Collisions page: the breadcrumb bar props and the resolved content
/// for the active kind (empty prompt, all-clear, or the two-pane view), as data.
pub(super) struct CollisionsPageModel {
    pub(super) breadcrumbs: BreadcrumbsProps,
    pub(super) content: ContentModel,
}

/// Computes the three collision models (memoised on the loaded keys and layout),
/// keeps each kind's selection valid, and shapes the breadcrumbs and active content.
pub(super) fn use_collisions_page(props: &CollisionsPageProps) -> CollisionsPageModel {
    let view_navigation = use_view_navigation();
    let selection = use_collision_selection();
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let kind = CollisionKind::from_query_param(props.kind.as_deref());
    let entry = props.entry.clone().filter(|value| !value.is_empty());
    let mut synced_route = use_synced_route();
    use_effect(use_reactive!(|(kind, entry)| {
        let view = AppView::Collisions { kind };
        view_navigation.restore_view(view);
        let mut selected = match kind {
            CollisionKind::Positions => selection.selected_island,
            CollisionKind::Hotkeys => selection.selected_hotkey_unit,
            CollisionKind::UnitPositions => selection.selected_unit_position,
        };
        if *selected.peek() != entry {
            selected.set(entry.clone());
        }
        let snapshot = NavSnapshot::Collisions { kind, entry };
        synced_route.set(snapshot);
    }));
    let islands_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        CollisionPageModel::compute(custom_keys)
    });
    let hotkey_units_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        let layout = *grid_layout.read();
        HotkeyCollisionPageModel::compute(custom_keys, layout)
    });
    let unit_positions_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        let layout = *grid_layout.read();
        UnitPositionPageModel::compute(custom_keys, layout)
    });
    let mut selected_island = selection.selected_island;
    let mut selected_hotkey_unit = selection.selected_hotkey_unit;
    let mut selected_unit_position = selection.selected_unit_position;
    use_effect(move || {
        let islands = islands_memo.read();
        if islands.is_empty() {
            return;
        }
        let current = selected_island.read().clone();
        let still_valid = match current {
            Some(ref key) => islands.iter().any(|island| island.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = islands.first().map(|island| island.key().to_owned());
            if let Some(key) = first_key {
                selected_island.set(Some(key));
            }
        }
    });
    use_effect(move || {
        let hotkey_units = hotkey_units_memo.read();
        if hotkey_units.is_empty() {
            return;
        }
        let current = selected_hotkey_unit.read().clone();
        let still_valid = match current {
            Some(ref key) => hotkey_units.iter().any(|unit| unit.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = hotkey_units.first().map(|unit| unit.key().to_owned());
            if let Some(key) = first_key {
                selected_hotkey_unit.set(Some(key));
            }
        }
    });
    use_effect(move || {
        let unit_positions = unit_positions_memo.read();
        if unit_positions.is_empty() {
            return;
        }
        let current = selected_unit_position.read().clone();
        let still_valid = match current {
            Some(ref key) => unit_positions.iter().any(|unit| unit.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = unit_positions.first().map(|unit| unit.key().to_owned());
            if let Some(key) = first_key {
                selected_unit_position.set(Some(key));
            }
        }
    });
    let islands = islands_memo();
    let island_count = islands.len();
    let has_file = loaded_keys.read().is_some();
    let sidebar_islands = islands.clone();
    let hotkey_units = hotkey_units_memo();
    let hotkey_unit_count = hotkey_units.len();
    let hotkey_collision_count = hotkey_units
        .iter()
        .map(|unit_view| unit_view.collision_count())
        .sum::<usize>();
    let sidebar_hotkey_units = hotkey_units.clone();
    let unit_positions = unit_positions_memo();
    let unit_position_unit_count = unit_positions.len();
    let unit_position_collision_count = unit_positions
        .iter()
        .map(|unit_view| unit_view.collision_count())
        .sum::<usize>();
    let sidebar_unit_positions = unit_positions.clone();
    let breadcrumbs = BreadcrumbsProps {
        kind,
        position_count: island_count,
        unit_position_count: unit_position_collision_count,
        hotkey_count: hotkey_collision_count,
        view_navigation,
    };
    let content = match kind {
        CollisionKind::Hotkeys => {
            if !has_file {
                let state = EmptyStateProps {
                    collision_kind: "hotkeys",
                    message: super::data::HOTKEYS_UPLOAD_PROMPT.to_owned(),
                };
                ContentModel::Empty(state)
            } else if hotkey_unit_count == 0 {
                let state = ClearStateProps {
                    collision_kind: "hotkeys",
                };
                ContentModel::Clear(state)
            } else {
                let sidebar = HotkeyUnitSidebarProps {
                    units: sidebar_hotkey_units,
                    selected_unit: selected_hotkey_unit,
                };
                let detail = HotkeyUnitDetailProps {
                    units: hotkey_units,
                    selected_unit: selected_hotkey_unit,
                    view_navigation,
                };
                let pane = HotkeysPane {
                    collision_kind: "hotkeys",
                    count: hotkey_unit_count,
                    sidebar,
                    detail,
                };
                ContentModel::Hotkeys(Box::new(pane))
            }
        }
        CollisionKind::UnitPositions => {
            if !has_file {
                let state = EmptyStateProps {
                    collision_kind: "unit-positions",
                    message: super::data::UNIT_POSITIONS_UPLOAD_PROMPT.to_owned(),
                };
                ContentModel::Empty(state)
            } else if unit_position_unit_count == 0 {
                let state = ClearStateProps {
                    collision_kind: "unit-positions",
                };
                ContentModel::Clear(state)
            } else {
                let sidebar = UnitPositionSidebarProps {
                    units: sidebar_unit_positions,
                    selected_unit: selected_unit_position,
                };
                let detail = UnitPositionDetailProps {
                    units: unit_positions,
                    selected_unit: selected_unit_position,
                    view_navigation,
                };
                let pane = UnitPositionsPane {
                    collision_kind: "unit-positions",
                    count: unit_position_unit_count,
                    sidebar,
                    detail,
                };
                ContentModel::UnitPositions(Box::new(pane))
            }
        }
        CollisionKind::Positions => {
            if !has_file {
                let state = EmptyStateProps {
                    collision_kind: "positions",
                    message: super::data::POSITIONS_UPLOAD_PROMPT.to_owned(),
                };
                ContentModel::Empty(state)
            } else if island_count == 0 {
                let state = ClearStateProps {
                    collision_kind: "positions",
                };
                ContentModel::Clear(state)
            } else {
                let sidebar = IslandSidebarProps {
                    islands: sidebar_islands,
                    selected_island,
                };
                let detail = IslandDetailProps {
                    islands,
                    selected_island,
                    view_navigation,
                };
                let pane = PositionsPane {
                    collision_kind: "positions",
                    count: island_count,
                    sidebar,
                    detail,
                };
                ContentModel::Positions(Box::new(pane))
            }
        }
    };
    CollisionsPageModel {
        breadcrumbs,
        content,
    }
}
