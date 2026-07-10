pub mod components;
mod hooks;
mod style;

use components::system_hotkeys_dialog::SystemHotkeysDialog;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_dialog_host;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(SystemHotkeysDialogHost);

/// Connects the system-hotkeys editor to app state and places it in the
/// always-mounted toolbar, so it opens from either the inline cog button or the
/// burger drawer. The dialog self-gates on the shared open signal.
#[component]
pub fn SystemHotkeysDialogHost() -> Element {
    let dialog = use_system_hotkeys_dialog_host();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysDialog { ..dialog }
        }
    }
}
