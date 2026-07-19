use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;

pub(super) struct SearchButtonHostModel {
    pub(super) open: bool,
    pub(super) aria_expanded: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_search_button() -> SearchButtonHostModel {
    // The open flag lives in the shared editor state so the mobile pager can see
    // it and freeze its scroll driven navigation while the dialog is open.
    let editor = use_editor_state();
    let mut open_signal = editor.search_dialog_open();
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_expanded = Some(open);
    SearchButtonHostModel {
        open,
        aria_expanded,
        onclick,
        on_open_change,
    }
}
