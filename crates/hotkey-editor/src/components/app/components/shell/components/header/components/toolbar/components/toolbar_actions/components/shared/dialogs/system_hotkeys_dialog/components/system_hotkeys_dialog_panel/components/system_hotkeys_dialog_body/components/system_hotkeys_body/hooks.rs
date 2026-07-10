use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The seam: this host reads the active category from context and picks which editor
/// to render. Each editor reads the editing section and drag follower from the dialog
/// state context itself and resolves its bindings from the CustomKeys query, so
/// nothing is threaded here.
pub(super) struct SystemHotkeysBodyModel {
    pub(super) active_category: Signal<SystemHotkeysCategory>,
}

pub(super) fn use_system_hotkeys_body() -> SystemHotkeysBodyModel {
    let state = use_system_hotkeys_dialog_state();
    let active_category = state.active_category();
    SystemHotkeysBodyModel { active_category }
}
