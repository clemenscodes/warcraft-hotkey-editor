mod hooks;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_undo_button;
use style::CLASS;

assert_component!(UndoButton);

/// Toolbar undo control. Reads the global [`UndoHistory`](crate::services::undo::UndoHistory)
/// from context and disables itself when the undo stack is empty. Its slot is hidden below
/// laptop, where the burger drawer offers undo instead.
#[component]
pub fn UndoButton() -> Element {
    let button = use_undo_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
    }
}
