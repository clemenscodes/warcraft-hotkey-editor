mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use presentation::{UndoButtonPresentation, use_undo_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline undo action: the toolbar button that undoes the last edit. It sources its own
/// disabled state and click handler from the undo history.
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
