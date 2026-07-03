use crate::components::shell::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::shared::icons::ICON_REDO;
use crate::services::undo::UndoHistory;
use dioxus::prelude::*;

/// Reads the global [`UndoHistory`] and shapes the toolbar button: disabled when
/// the redo stack is empty, clicking replays one step.
pub(super) fn use_redo_button() -> ToolbarButtonProps {
    let history = use_context::<UndoHistory>();
    let disabled = !history.can_redo();
    let onclick = EventHandler::new(move |_event: MouseEvent| history.redo());
    ToolbarButtonProps {
        icon: ICON_REDO,
        aria_label: "Redo",
        disabled,
        data_action: Some("redo"),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
