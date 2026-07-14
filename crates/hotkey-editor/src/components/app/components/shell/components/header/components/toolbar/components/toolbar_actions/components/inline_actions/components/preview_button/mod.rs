mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::preview_dialog::PreviewDialog;
use dioxus::prelude::*;
use presentation::{PreviewButtonPresentation, use_preview_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline preview action: the toolbar button that opens the export-preview pane, and the
/// pane dialog it owns, mounted beneath it. It owns the pane's open signal locally, so the
/// dialog is part of the button and travels with it.
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
