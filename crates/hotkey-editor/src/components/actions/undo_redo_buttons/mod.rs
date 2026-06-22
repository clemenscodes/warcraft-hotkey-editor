use dioxus::prelude::*;

use crate::components::shared::icons::{ICON_REDO, ICON_UNDO};
use crate::components::shell::header::{TOOLBAR_BTN_CLASS, TOOLBAR_ICON_CLASS};
use crate::services::undo::UndoHistory;

/// Toolbar undo/redo controls. Reads the global [`UndoHistory`] from context;
/// each button disables itself when its stack is empty. The keyboard shortcuts
/// (Ctrl/Cmd+Z, Ctrl/Cmd+Shift+Z) drive the same history.
#[component]
pub(crate) fn UndoRedoButtons() -> Element {
    let history = use_context::<UndoHistory>();
    let can_undo = history.can_undo();
    let can_redo = history.can_redo();
    let trigger_undo = move |_| history.undo();
    let trigger_redo = move |_| history.redo();

    rsx! {
        button {
            class: TOOLBAR_BTN_CLASS,
            r#type: "button",
            aria_label: "Undo",
            "data-action": "undo",
            disabled: !can_undo,
            onclick: trigger_undo,
            span {
                class: TOOLBAR_ICON_CLASS,
                aria_hidden: "true",
                dangerous_inner_html: ICON_UNDO,
            }
        }
        button {
            class: TOOLBAR_BTN_CLASS,
            r#type: "button",
            aria_label: "Redo",
            "data-action": "redo",
            disabled: !can_redo,
            onclick: trigger_redo,
            span {
                class: TOOLBAR_ICON_CLASS,
                aria_hidden: "true",
                dangerous_inner_html: ICON_REDO,
            }
        }
    }
}
