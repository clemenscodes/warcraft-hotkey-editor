pub mod components;
mod presentation;
mod style;

use components::help_dialog::HelpDialog;
use dioxus::prelude::*;
use presentation::use_help_dialog_host;
use style::CLASS;
use tw_macro::assert_component;

/// Connects the onboarding help dialog to app state and places it in the
/// always-mounted toolbar, so it opens from either the inline help button or the
/// burger drawer (and auto-opens on first visit). The dialog self-gates on the shared
/// open signal.
#[component]
pub fn HelpDialogHost() -> Element {
    let help_open = use_help_dialog_host();
    rsx! {
        div {
            class: CLASS,
            HelpDialog { help_open }
        }
    }
}

assert_component!(HelpDialogHost);
