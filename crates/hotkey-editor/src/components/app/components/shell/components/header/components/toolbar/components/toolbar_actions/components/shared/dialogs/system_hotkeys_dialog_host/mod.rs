pub mod components;
mod presentation;
mod style;

use components::system_hotkeys_dialog::SystemHotkeysDialog;
use dioxus::prelude::*;
use presentation::{SystemHotkeysDialogHostModel, use_system_hotkeys_dialog_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connects the system-hotkeys editor to app state and places it in the
/// always-mounted toolbar, so it opens from either the inline cog button or the
/// burger drawer. The dialog self-gates on the shared open value.
#[component]
pub fn SystemHotkeysDialogHost() -> Element {
    let SystemHotkeysDialogHostModel {
        open,
        on_open_change,
    } = use_system_hotkeys_dialog_host();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysDialog { open, on_open_change }
        }
    }
}

assert_component!(SystemHotkeysDialogHost);
