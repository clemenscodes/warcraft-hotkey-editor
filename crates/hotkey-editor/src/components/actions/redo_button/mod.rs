mod hooks;

use crate::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_redo_button;

/// Toolbar redo control. Reads the global [`UndoHistory`] from context and
/// disables itself when the redo stack is empty.
#[component]
pub fn RedoButton() -> Element {
    let button = use_redo_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
