mod hooks;
mod style;

use super::shared::dialogs::layout_editor::LayoutEditor;
use crate::assert_component;
use dioxus::prelude::*;
use hooks::use_layout_editor_host;
use style::CLASS;

assert_component!(LayoutEditorHost);

/// Connects the global hotkey-layout editor to app state and places it in the
/// always-mounted toolbar, so it opens from either the centered grid-layout button
/// (laptop and up) or the burger drawer. The dialog self-gates on the shared open
/// signal.
#[component]
pub fn LayoutEditorHost() -> Element {
    let dialog = use_layout_editor_host();
    rsx! {
        div {
            class: CLASS,
            LayoutEditor { ..dialog }
        }
    }
}
