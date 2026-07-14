use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_UNDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

pub(super) struct UndoButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_undo_button() -> UndoButtonPresentation {
    let history = use_undo_history();
    let can_undo = history.can_undo();
    let disabled = !can_undo;
    let onclick = EventHandler::new(move |_event: MouseEvent| history.undo());
    UndoButtonPresentation {
        icon: ICON_UNDO,
        aria_label: ARIA_LABEL,
        disabled,
        onclick,
    }
}
