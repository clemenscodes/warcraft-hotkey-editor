use crate::components::shared::icons::ICON_UNDO;
use crate::components::shared::toolbar_button::ToolbarButtonProps;
use crate::services::undo::UndoHistory;
use dioxus::prelude::*;

/// Reads the global [`UndoHistory`] and shapes the toolbar button: disabled when
/// the undo stack is empty, clicking pops one step.
pub(super) fn use_undo_button() -> ToolbarButtonProps {
    let history = use_context::<UndoHistory>();
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
