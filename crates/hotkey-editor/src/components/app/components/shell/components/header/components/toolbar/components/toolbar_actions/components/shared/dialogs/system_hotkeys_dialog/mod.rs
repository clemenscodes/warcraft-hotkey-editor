pub mod components;
mod hooks;
mod logic;
mod props;
pub mod state;
mod style;

use components::system_hotkeys_dialog_panel::SystemHotkeysDialogPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_system_hotkeys_dialog;
use logic::SystemHotkeysDialogShell;
pub use props::SystemHotkeysDialogProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(SystemHotkeysDialog);

/// Edits Warcraft III's system and menu hotkeys. It owns its own dialog shell: the
/// hook holds the UI signals, the shell struct shapes the panel, and this places the
/// panel inside its own backdrop `div` (the dimmed, centring layer) within the library
/// `DialogRoot`. No project class touches the library element.
#[component]
pub fn SystemHotkeysDialog(props: SystemHotkeysDialogProps) -> Element {
    use_body_scroll_lock(props.system_hotkeys_open);
    let model = use_system_hotkeys_dialog(&props);
    let SystemHotkeysDialogShell {
        open,
        on_open_change,
        panel,
    } = SystemHotkeysDialogShell::from(&model);
    if !open {
        return rsx! {};
    }
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                SystemHotkeysDialogPanel { ..panel }
            }
        }
    }
}
