use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::UnitMode;

/// The two inputs the presentational `ModeTabs` needs: the active mode (to mark the
/// current button) and the select handler that dispatches the domain cascade. This is
/// the host's shaped domain data — never the tabs' own props type.
pub(super) struct ModeTabsInputs {
    pub(super) unit_mode: Signal<UnitMode>,
    pub(super) on_select: EventHandler<UnitMode>,
}

/// The seam: source the active mode, and wire the select handler to the navigation
/// service's `select_mode` cascade (set mode → default unit → clear slot). The
/// presentational `ModeTabs` gets only those two inputs.
pub(super) fn use_mode_tabs_host() -> ModeTabsInputs {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let unit_mode = navigation.unit_mode();
    let selected_slot = editor.selected_slot();
    let on_select = EventHandler::new(move |mode: UnitMode| {
        navigation.select_mode(mode, selected_slot);
    });
    ModeTabsInputs {
        unit_mode,
        on_select,
    }
}
