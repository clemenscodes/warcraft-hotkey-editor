use crate::components::app::components::shell::components::shared::icons::ICON_UNDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// The undo button's shaped view: its icon, label, disabled state, and click handler.
pub(super) struct UndoButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the global [`UndoHistory`](crate::services::undo::UndoHistory) and shapes the toolbar button: disabled when
/// the undo stack is empty, clicking pops one step.
pub(super) fn use_undo_button() -> UndoButtonModel {
    let history = use_undo_history();
    let disabled = !history.can_undo();
    let onclick = EventHandler::new(move |_event: MouseEvent| history.undo());
    UndoButtonModel {
        icon: ICON_UNDO,
        aria_label: "Undo",
        disabled,
        onclick,
    }
}
