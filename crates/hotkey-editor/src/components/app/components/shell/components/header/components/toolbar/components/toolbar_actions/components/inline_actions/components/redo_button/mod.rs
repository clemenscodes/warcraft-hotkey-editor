mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use presentation::{RedoButtonPresentation, use_redo_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline redo action: the toolbar button that redoes the next edit. It sources its own
/// disabled state and click handler from the undo history.
#[component]
pub fn RedoButton() -> Element {
    let RedoButtonPresentation {
        icon,
        aria_label,
        disabled,
        onclick,
    } = use_redo_button();
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

assert_component!(RedoButton);
