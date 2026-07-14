use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_REDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// The redo button's shaped data: the fixed icon and label, whether it is disabled (nothing to
/// redo), and the click handler that redoes the next edit.
pub(super) struct RedoButtonPresentation {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) disabled: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the undo history and shapes the button's data: disabled while there is nothing to redo,
/// and the click handler that redoes the next edit.
pub(super) fn use_redo_button() -> RedoButtonPresentation {
    let history = use_undo_history();
    let can_redo = history.can_redo();
    let disabled = !can_redo;
    let onclick = EventHandler::new(move |_event: MouseEvent| history.redo());
    RedoButtonPresentation {
        icon: ICON_REDO,
        aria_label: ARIA_LABEL,
        disabled,
        onclick,
    }
}
