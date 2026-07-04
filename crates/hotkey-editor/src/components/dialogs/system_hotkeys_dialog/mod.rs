pub mod components;
mod hooks;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_system_hotkeys_dialog;
pub use props::SystemHotkeysDialogProps;

/// Edits Warcraft III's system and menu hotkeys. A variant of the `Dialog` base:
/// the hook holds the UI signals, and the body composes the shell with the
/// category breadcrumbs, the active category's editor, and the inventory drag
/// follower.
#[component]
pub fn SystemHotkeysDialog(props: SystemHotkeysDialogProps) -> Element {
    let model = use_system_hotkeys_dialog(&props);
    let system_hotkeys_open = props.system_hotkeys_open;
    if !system_hotkeys_open() {
        return rsx! {};
    }
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
