use super::components::mode_tabs::ModeTabsProps;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_database::UnitMode;

/// The seam: source the active mode, and wire the select handler to the navigation
/// service's `select_mode` cascade (set mode → default unit → clear slot). The
/// presentational `ModeTabs` gets only those two inputs.
pub(super) fn use_mode_tabs_host() -> ModeTabsProps {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let unit_mode = navigation.unit_mode();
    let selected_slot = editor.selected_slot();
    let on_select = EventHandler::new(move |mode: UnitMode| {
        navigation.select_mode(mode, selected_slot);
    });
    ModeTabsProps {
        unit_mode,
        on_select,
    }
}
