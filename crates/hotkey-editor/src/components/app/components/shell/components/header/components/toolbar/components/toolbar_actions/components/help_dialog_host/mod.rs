mod hooks;
mod style;

use super::shared::dialogs::help_dialog::HelpDialog;
use crate::assert_component;
use dioxus::prelude::*;
use hooks::use_help_dialog_host;
use style::CLASS;

assert_component!(HelpDialogHost);

/// Connects the onboarding help dialog to app state and places it in the
/// always-mounted toolbar, so it opens from either the inline help button or the
/// burger drawer (and auto-opens on first visit). The dialog self-gates on the shared
/// open signal.
#[component]
pub fn HelpDialogHost() -> Element {
    let dialog = use_help_dialog_host();
    rsx! {
        div {
            class: CLASS,
            HelpDialog { ..dialog }
        }
    }
}
