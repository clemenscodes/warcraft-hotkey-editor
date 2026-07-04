use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::app::components::shell::components::shared::icons::ICON_UNDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// Reads the global [`UndoHistory`](crate::services::undo::UndoHistory) and shapes the toolbar button: disabled when
/// the undo stack is empty, clicking pops one step.
pub(super) fn use_undo_button() -> ToolbarButtonProps {
    let history = use_undo_history();
    let disabled = !history.can_undo();
    let onclick = EventHandler::new(move |_event: MouseEvent| history.undo());
    ToolbarButtonProps {
        icon: ICON_UNDO,
        aria_label: "Undo",
        disabled,
        data_action: Some("undo"),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
