mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_redo_button;
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar redo control. Reads the global [`UndoHistory`](crate::services::undo::UndoHistory)
/// from context and disables itself when the redo stack is empty. Its slot is hidden below
/// laptop, where the burger drawer offers redo instead.
#[component]
pub fn RedoButton() -> Element {
    let button = use_redo_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
    }
}

assert_component!(RedoButton);
