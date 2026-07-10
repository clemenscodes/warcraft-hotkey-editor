pub mod components;
mod hooks;
mod style;

use components::preview_dialog::PreviewDialog;
use dioxus::prelude::*;
use hooks::use_preview_dialog_host;
use style::CLASS;
use tw_macro::assert_component;

/// Connects the serialized-keys preview dialog to app state and places it in the
/// always-mounted toolbar, so it opens from either the inline preview button (laptop
/// and up) or the burger drawer (below), which merely flip the shared open signal.
/// The dialog self-gates on that signal, so this host renders nothing until it opens.
#[component]
pub fn PreviewDialogHost() -> Element {
    let dialog = use_preview_dialog_host();
    rsx! {
        div {
            class: CLASS,
            PreviewDialog { ..dialog }
        }
    }
}

assert_component!(PreviewDialogHost);
