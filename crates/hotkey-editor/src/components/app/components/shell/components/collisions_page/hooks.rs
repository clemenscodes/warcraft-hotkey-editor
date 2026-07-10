use dioxus::prelude::*;

use super::components::body::ContentModel;
use super::logic::{
    CollisionBreadcrumbsInputs, compute_hotkey_unit_views, compute_island_views,
    compute_unit_position_views,
};
use super::model::{
    CollisionEntry, CollisionList, HotkeysContent, PositionsContent, UnitPositionsContent,
};
use super::props::CollisionsPageProps;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use crate::services::collision_selection::CollisionSelection;
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::grid_layout::context::use_grid_layout;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::context::{use_synced_route, use_view_navigation};
use crate::services::navigation::nav_snapshot::NavSnapshot;
use crate::services::navigation::view_navigation::ViewNavigationContext;

/// The shaped Collisions page: the breadcrumb bar props and the resolved content
/// for the active kind (empty prompt, all-clear, or the two-pane view), as data.
pub(super) struct CollisionsPageModel {
    pub(super) breadcrumbs: Vec<BreadcrumbView>,
    pub(super) content: ContentModel,
}

/// Restores the active view, mirrors the incoming `?entry=` into the active kind's
/// selection signal, and publishes the route snapshot the shell's URL sync reads.
/// Reactive on the route's `kind` and `entry` so it re-runs only when they change.
fn use_route_sync(
    kind: CollisionKind,
    entry: Option<String>,
    view_navigation: ViewNavigationContext,
    selection: CollisionSelection,
) {
    let mut synced_route = use_synced_route();
    use_effect(use_reactive!(|(kind, entry)| {
        let view = AppView::Collisions { kind };
        view_navigation.restore_view(view);
        let mut selected = match kind {
            CollisionKind::Positions => selection.selected_island(),
            CollisionKind::Hotkeys => selection.selected_hotkey_unit(),
            CollisionKind::UnitPositions => selection.selected_unit_position(),
        };
        if *selected.peek() != entry {
            selected.set(entry.clone());
        }
        let snapshot = NavSnapshot::Collisions { kind, entry };
        synced_route.set(snapshot);
    }));
}

/// Keeps one kind's selection pointing at a live entry: when the list is non-empty
/// and the current selection is missing or stale, it falls back to the first entry.
/// Applied once per kind, replacing three hand-copied effects.
fn use_valid_selection<View>(memo: Memo<Vec<View>>, selected: Signal<Option<String>>)
where
    View: CollisionEntry + PartialEq + 'static,
{
    let mut selected = selected;
    use_effect(move || {
        let views = memo.read();
        if views.is_empty() {
            return;
        }
        let current = selected.read().clone();
        let still_valid = match current {
            Some(ref key) => views.iter().any(|view| view.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = views.first().map(|view| view.key().to_owned());
            if let Some(key) = first_key {
                selected.set(Some(key));
            }
        }
    });
}

/// Computes the three collision models (memoised on the loaded keys and layout),
/// keeps each kind's selection valid, and shapes the breadcrumbs and active content.
pub(super) fn use_collisions_page(props: &CollisionsPageProps) -> CollisionsPageModel {
    let view_navigation = use_view_navigation();
    let selection = use_collision_selection();
    let custom_keys_service = use_custom_keys_service();
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let kind = CollisionKind::from_query_param(props.kind.as_deref());
    let entry = props.entry.clone().filter(|value| !value.is_empty());
    use_route_sync(kind, entry, view_navigation, selection);

    let islands_memo = use_memo(move || {
        let report = custom_keys_service.cross_unit_collisions();
        compute_island_views(&report)
    });
    let hotkey_units_memo = use_memo(move || {
        let layout = *grid_layout.read();
        let report = custom_keys_service.unit_collisions(layout);
        compute_hotkey_unit_views(&report)
    });
    let unit_positions_memo = use_memo(move || {
        let layout = *grid_layout.read();
        let report = custom_keys_service.unit_collisions(layout);
        compute_unit_position_views(&report)
    });

    let selected_island = selection.selected_island();
    let selected_hotkey_unit = selection.selected_hotkey_unit();
    let selected_unit_position = selection.selected_unit_position();
    use_valid_selection(islands_memo, selected_island);
    use_valid_selection(hotkey_units_memo, selected_hotkey_unit);
    use_valid_selection(unit_positions_memo, selected_unit_position);

    let island_views = islands_memo();
    let hotkey_unit_views = hotkey_units_memo();
    let unit_position_views = unit_positions_memo();
    let islands = CollisionList::from(island_views);
    let hotkey_units = CollisionList::from(hotkey_unit_views);
    let unit_positions = CollisionList::from(unit_position_views);
    let has_file = loaded_keys.read().is_some();

    let breadcrumb_inputs = CollisionBreadcrumbsInputs {
        active_kind: kind,
        position_count: islands.unit_count,
        unit_position_count: unit_positions.collision_count,
        hotkey_count: hotkey_units.collision_count,
        view_navigation,
    };
    let breadcrumbs = breadcrumb_inputs.into_views();
    let content = match kind {
        CollisionKind::Hotkeys => {
            let inputs = HotkeysContent {
                has_file,
                list: hotkey_units,
            };
            ContentModel::from(inputs)
        }
        CollisionKind::UnitPositions => {
            let inputs = UnitPositionsContent {
                has_file,
                list: unit_positions,
            };
            ContentModel::from(inputs)
        }
        CollisionKind::Positions => {
            let inputs = PositionsContent {
                has_file,
                list: islands,
            };
            ContentModel::from(inputs)
        }
    };
    CollisionsPageModel {
        breadcrumbs,
        content,
    }
}
