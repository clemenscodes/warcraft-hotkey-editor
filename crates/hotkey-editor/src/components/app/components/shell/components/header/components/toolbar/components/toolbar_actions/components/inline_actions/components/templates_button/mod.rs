mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::TemplatesDialog;
use dioxus::prelude::*;
use presentation::{TemplatesButtonPresentation, use_templates_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline templates action: the toolbar button that opens the layout-templates browser, and
/// that browser dialog, mounted beneath it. It owns the browser's open signal locally, so the
/// dialog is part of the button and travels with it.
#[component]
pub fn TemplatesButton() -> Element {
    let TemplatesButtonPresentation {
        icon,
        aria_label,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    } = use_templates_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                aria_haspopup,
                aria_expanded,
                onclick,
            }
            TemplatesDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(TemplatesButton);
