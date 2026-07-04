mod hooks;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
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
