use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use warcraft_api::UnitMode;
use warcraft_api::UnitModeSelection;

pub(super) struct ModeTabsInputs {
    pub(super) unit_modes: Signal<UnitModeSelection>,
    pub(super) on_select: EventHandler<UnitMode>,
}

pub(super) fn use_mode_tabs_host() -> ModeTabsInputs {
    let navigation = use_view_navigation();
    let editor = use_editor_state();
    let unit_modes = navigation.unit_modes();
    let selected_slot = editor.selected_slot();
    let on_select = EventHandler::new(move |mode: UnitMode| {
        navigation.toggle_mode(mode, selected_slot);
    });
    ModeTabsInputs {
        unit_modes,
        on_select,
    }
}
