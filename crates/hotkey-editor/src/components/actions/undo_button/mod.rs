use dioxus::prelude::*;

use crate::components::shared::icons::ICON_UNDO;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::undo::UndoHistory;

/// Toolbar undo control. Reads the global [`UndoHistory`] from context and
/// disables itself when the undo stack is empty.
#[component]
pub fn UndoButton() -> Element {
    let history = use_context::<UndoHistory>();
    let can_undo = history.can_undo();
    let undo_disabled = !can_undo;
    let trigger_undo = move |_| history.undo();
    rsx! {
        ToolbarButton {
            icon: ICON_UNDO,
            aria_label: "Undo",
            "data-action": "undo",
            disabled: undo_disabled,
            onclick: trigger_undo,
        }
    }
}
