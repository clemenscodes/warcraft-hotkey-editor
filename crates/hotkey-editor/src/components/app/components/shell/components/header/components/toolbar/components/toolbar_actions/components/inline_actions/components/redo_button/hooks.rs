use crate::components::app::components::shell::components::shared::icons::ICON_REDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// The redo button's shaped view: its icon, label, disabled state, and click handler.
pub(super) struct RedoButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the global [`UndoHistory`](crate::services::undo::UndoHistory) and shapes the toolbar button: disabled when
/// the redo stack is empty, clicking replays one step.
pub(super) fn use_redo_button() -> RedoButtonModel {
    let history = use_undo_history();
    let disabled = !history.can_redo();
    let onclick = EventHandler::new(move |_event: MouseEvent| history.redo());
    RedoButtonModel {
        icon: ICON_REDO,
        aria_label: "Redo",
        disabled,
        onclick,
    }
}
