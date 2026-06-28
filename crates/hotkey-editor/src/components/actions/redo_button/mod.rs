use dioxus::prelude::*;

use crate::components::shared::icons::ICON_REDO;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::undo::UndoHistory;

/// Toolbar redo control. Reads the global [`UndoHistory`] from context and
/// disables itself when the redo stack is empty.
#[component]
pub fn RedoButton() -> Element {
    let history = use_context::<UndoHistory>();
    let can_redo = history.can_redo();
    let redo_disabled = !can_redo;
    let trigger_redo = move |_| history.redo();
    rsx! {
        ToolbarButton {
            icon: ICON_REDO,
            aria_label: "Redo",
            "data-action": "redo",
            disabled: redo_disabled,
            onclick: trigger_redo,
        }
    }
}
