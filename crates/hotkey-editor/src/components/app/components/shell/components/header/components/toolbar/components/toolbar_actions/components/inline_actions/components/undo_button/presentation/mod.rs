use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_UNDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// The undo button's shaped data: the fixed icon and label, whether it is disabled (nothing to
/// undo), and the click handler that undoes the last edit.
pub(super) struct UndoButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the undo history and shapes the button's data: disabled while there is nothing to undo,
/// and the click handler that undoes the last edit.
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
