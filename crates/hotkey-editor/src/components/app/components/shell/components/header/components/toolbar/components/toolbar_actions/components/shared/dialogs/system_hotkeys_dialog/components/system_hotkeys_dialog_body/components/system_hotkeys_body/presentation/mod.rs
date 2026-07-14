use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

pub(super) struct SystemHotkeysBodyModel {
    pub(super) active_category: Signal<SystemHotkeysCategory>,
}

pub(super) fn use_system_hotkeys_body() -> SystemHotkeysBodyModel {
    let state = use_system_hotkeys_dialog_state();
    let active_category = state.active_category();
    SystemHotkeysBodyModel { active_category }
}
