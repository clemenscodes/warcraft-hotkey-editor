use super::components::editor_tabs_bar::EditorTabsBarProps;
use super::components::editor_workspace::EditorWorkspaceProps;
use super::props::EditorPageProps;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::{use_synced_route, use_view_navigation};
use crate::services::navigation::editor_nav::DecodedEditorNav;
use crate::services::navigation::nav_snapshot::NavSnapshot;
use dioxus::prelude::*;

/// The editor page, shaped for its two children: the tab bar and the workspace. Built
/// from context (the navigation signals, the editor's UI state, the loaded document,
/// the grid layout) so the page body is a flat compose of already-shaped props.
pub(super) struct EditorPageModel {
    pub(super) tabs: EditorTabsBarProps,
    pub(super) workspace: EditorWorkspaceProps,
}

/// Reconcile the editor route into the shell's navigation signals, then read every
/// editor signal from context and shape the two children's props. The reconcile is
/// the read side of the URL contract — decoding `?race=&mode=&unit=&q=` and writing it
/// into the navigation signals whenever the route changes (deep-link, back/forward) —
/// while the shell's push effect handles the write side.
pub(super) fn use_editor_page(props: &EditorPageProps) -> EditorPageModel {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let mut synced_route = use_synced_route();
    let decoded = DecodedEditorNav::decode(
        props.race.as_deref(),
        props.mode.as_deref(),
        props.unit.as_deref(),
        props.q.as_deref(),
    );
    use_effect(use_reactive!(|decoded| {
        navigation.restore(AppView::Editor, &decoded);
        let snapshot = NavSnapshot::Editor(decoded.clone());
        synced_route.set(snapshot);
    }));
    let tabs = EditorTabsBarProps {
        unit_mode: navigation.unit_mode,
        active_race: navigation.active_race,
        selected_unit_id: navigation.selected_unit_id,
        selected_slot: editor.selected_slot,
    };
    let workspace = EditorWorkspaceProps {
        active_race: navigation.active_race,
        unit_mode: navigation.unit_mode,
        selected_unit_id: navigation.selected_unit_id,
        selected_slot: editor.selected_slot,
        search_query: navigation.search_query,
        search_field: editor.search_field,
        show_abilityless_units: editor.show_abilityless_units,
        expand_variants: editor.expand_variants,
        collapsed_categories: editor.collapsed_categories,
        selected_from_research: editor.selected_from_research,
        selected_from_uprooted: editor.selected_from_uprooted,
        tier_overrides: editor.tier_overrides,
        dragging_slot: editor.dragging_slot,
        drop_target_tile: editor.drop_target_tile,
        drag_follower: editor.drag_follower,
        loaded_keys,
        grid_layout,
        update_hotkeys_on_move: editor.update_hotkeys_on_move,
        hotkey_assign_request: editor.hotkey_assign_request,
    };
    EditorPageModel { tabs, workspace }
}
