mod hooks;

use crate::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_undo_button;

/// Toolbar undo control. Reads the global [`UndoHistory`] from context and
/// disables itself when the undo stack is empty.
#[component]
pub fn UndoButton() -> Element {
    let button = use_undo_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
