pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::system_hotkeys_dialog_body::SystemHotkeysDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_system_hotkeys_dialog;
use logic::SystemHotkeysDialogShell;
pub use props::SystemHotkeysDialogProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(SystemHotkeysDialog);

/// Edits Warcraft III's system and menu hotkeys. It owns its own dialog shell: the
/// hook holds the UI signals, the shell struct names the header and scroll body, and
/// this places them inside the backdrop and bordered box.
#[component]
pub fn SystemHotkeysDialog(props: SystemHotkeysDialogProps) -> Element {
    use_body_scroll_lock(props.system_hotkeys_open);
    let model = use_system_hotkeys_dialog(&props);
    let SystemHotkeysDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = SystemHotkeysDialogShell::from(&model);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                SystemHotkeysDialogBody { ..body }
            }
        }
    }
}
