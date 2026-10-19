use crate::components::app::components::shell::components::shared::dialog_toggle::use_dialog_toggle;
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;

pub(super) struct SearchButtonHostModel {
    pub(super) open: bool,
    pub(super) aria_expanded: Option<bool>,
    pub(super) onpointerdown: EventHandler<PointerEvent>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_search_button() -> SearchButtonHostModel {
    // The open flag lives in the shared editor state so the mobile pager can see
    // it and freeze its scroll driven navigation while the dialog is open.
    let editor = use_editor_state();
    let open_signal = editor.search_dialog_open();
    let open = open_signal();
    // The host remembers on pointerdown whether the dialog was open, because a
    // press outside the open dialog dismisses it before this click arrives.
    let toggle = use_dialog_toggle(open_signal);
    let onpointerdown = toggle.onpointerdown();
    let onclick = toggle.onclick();
    let on_open_change = toggle.on_open_change();
    let aria_expanded = Some(open);
    SearchButtonHostModel {
        open,
        aria_expanded,
        onpointerdown,
        onclick,
        on_open_change,
    }
}
