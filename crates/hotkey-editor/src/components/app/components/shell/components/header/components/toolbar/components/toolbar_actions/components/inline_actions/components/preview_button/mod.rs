mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::preview_dialog::PreviewDialog;
use dioxus::prelude::*;
use presentation::{PreviewButtonPresentation, use_preview_button};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PreviewButton() -> Element {
    let PreviewButtonPresentation {
        icon,
        aria_label,
        aria_pressed,
        open,
        onclick,
        on_open_change,
    } = use_preview_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                aria_pressed,
                onclick,
            }
            PreviewDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(PreviewButton);
