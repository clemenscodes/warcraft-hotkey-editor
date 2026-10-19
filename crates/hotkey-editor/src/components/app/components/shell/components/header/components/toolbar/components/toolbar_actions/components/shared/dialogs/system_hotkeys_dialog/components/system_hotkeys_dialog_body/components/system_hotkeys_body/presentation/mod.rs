use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

pub(super) struct SystemHotkeysBodyPresentation {
    pub(super) category: SystemHotkeysCategory,
}

pub(super) fn use_system_hotkeys_body() -> SystemHotkeysBodyPresentation {
    let state = use_system_hotkeys_dialog_state();
    let active_category = state.active_category();
    let category = *active_category.read();
    SystemHotkeysBodyPresentation { category }
}
