mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::HelpDialog;
use dioxus::prelude::*;
use presentation::{HelpButtonPresentation, use_help_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline help action: the toolbar button that opens the onboarding guide, and the guide
/// dialog it owns, mounted beneath it. It owns the guide's open signal locally (opening it on
/// a first visit), so the dialog is part of the button and travels with it — move this button
/// and its dialog comes along.
#[component]
pub fn HelpButton() -> Element {
    let HelpButtonPresentation {
        icon,
        aria_label,
        aria_haspopup,
        aria_expanded,
        open,
        onclick,
        on_open_change,
    } = use_help_button();
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
            HelpDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(HelpButton);
