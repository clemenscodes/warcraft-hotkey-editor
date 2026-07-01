use super::collision_breadcrumbs::CollisionBreadcrumbsProps;
use super::collisions_clear_state::CollisionsClearState;
use super::collisions_content::CollisionsContent;
use super::collisions_empty_state::CollisionsEmptyState;
use super::hotkey_unit_detail::HotkeyUnitDetail;
use super::hotkey_unit_sidebar::HotkeyUnitSidebar;
use super::island_detail::IslandDetail;
use super::island_sidebar::IslandSidebar;
use super::props::CollisionsPageProps;
use super::unit_position_detail::UnitPositionDetail;
use super::unit_position_sidebar::UnitPositionSidebar;
use super::{CollisionPageModel, HotkeyCollisionPageModel, UnitPositionPageModel};
use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// The shaped Collisions page: the breadcrumb bar props and the resolved content
/// for the active kind (empty prompt, all-clear, or the two-pane view).
pub(super) struct CollisionsPageModel {
    pub(super) breadcrumbs: CollisionBreadcrumbsProps,
    pub(super) content: Element,
}

/// Computes the three collision models (memoised on the loaded keys and layout),
/// keeps each kind's selection valid, and shapes the breadcrumbs and active content.
pub(super) fn use_collisions_page(props: &CollisionsPageProps) -> CollisionsPageModel {
    let kind = props.kind;
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let view_navigation = use_context::<ViewNavigationContext>();
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
    let mut selected_island = props.selected_island;
    let mut selected_hotkey_unit = props.selected_hotkey_unit;
    let mut selected_unit_position = props.selected_unit_position;
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
    let breadcrumbs = CollisionBreadcrumbsProps {
        kind,
        position_count: island_count,
        unit_position_count: unit_position_collision_count,
        hotkey_count: hotkey_collision_count,
        view_navigation,
    };
    let content = match kind {
        CollisionKind::Hotkeys => {
            if !has_file {
                rsx! {
                    CollisionsEmptyState {
                        collision_kind: "hotkeys",
                        message: "Upload your CustomKeys.txt to inspect hotkey collisions.",
                    }
                }
            } else if hotkey_unit_count == 0 {
                rsx! {
                    CollisionsClearState { collision_kind: "hotkeys" }
                }
            } else {
                rsx! {
                    CollisionsContent {
                        collision_kind: "hotkeys",
                        count: hotkey_unit_count,
                        HotkeyUnitSidebar {
                            units: sidebar_hotkey_units,
                            selected_unit: selected_hotkey_unit,
                        }
                        HotkeyUnitDetail {
                            units: hotkey_units,
                            selected_unit: selected_hotkey_unit,
                            view_navigation,
                        }
                    }
                }
            }
        }
        CollisionKind::UnitPositions => {
            if !has_file {
                rsx! {
                    CollisionsEmptyState {
                        collision_kind: "unit-positions",
                        message: "Upload your CustomKeys.txt to inspect unit position collisions.",
                    }
                }
            } else if unit_position_unit_count == 0 {
                rsx! {
                    CollisionsClearState { collision_kind: "unit-positions" }
                }
            } else {
                rsx! {
                    CollisionsContent {
                        collision_kind: "unit-positions",
                        count: unit_position_unit_count,
                        UnitPositionSidebar {
                            units: sidebar_unit_positions,
                            selected_unit: selected_unit_position,
                        }
                        UnitPositionDetail {
                            units: unit_positions,
                            selected_unit: selected_unit_position,
                            view_navigation,
                        }
                    }
                }
            }
        }
        CollisionKind::Positions => {
            if !has_file {
                rsx! {
                    CollisionsEmptyState {
                        collision_kind: "positions",
                        message: "Upload your CustomKeys.txt to inspect position collisions.",
                    }
                }
            } else if island_count == 0 {
                rsx! {
                    CollisionsClearState { collision_kind: "positions" }
                }
            } else {
                rsx! {
                    CollisionsContent {
                        collision_kind: "positions",
                        count: island_count,
                        IslandSidebar { islands: sidebar_islands, selected_island }
                        IslandDetail { islands, selected_island, view_navigation }
                    }
                }
            }
        }
    };
    CollisionsPageModel {
        breadcrumbs,
        content,
    }
}
