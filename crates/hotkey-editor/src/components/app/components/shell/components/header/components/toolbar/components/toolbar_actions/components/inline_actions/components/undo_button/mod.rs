mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use presentation::{UndoButtonPresentation, use_undo_button};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UndoButton() -> Element {
    let UndoButtonPresentation {
        icon,
        aria_label,
        disabled,
        onclick,
    } = use_undo_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                disabled,
                onclick,
            }
        }
    }
}

assert_component!(UndoButton);
