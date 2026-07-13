pub mod components;
mod presentation;
pub mod state;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::system_hotkeys_dialog_body::SystemHotkeysDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use presentation::SystemHotkeysDialogShell;
use presentation::use_system_hotkeys_dialog;
use tw_macro::assert_component;

/// Edits Warcraft III's system and menu hotkeys, placed in the always-mounted toolbar so
/// it opens from either the inline cog button or the burger drawer. The presentation reads
/// the shared open signal from context and provides the dialog's UI state (active category,
/// editing section, inventory drag) to the subtree; this renders the reusable
/// `WarcraftDialog` with the isolated body region only while open, and the headless dialog
/// derives its own close from `on_open_change`.
#[component]
pub fn SystemHotkeysDialog() -> Element {
    let model = use_system_hotkeys_dialog();
    let SystemHotkeysDialogShell {
        open,
        on_open_change,
        title,
    } = SystemHotkeysDialogShell::from(&model);
    let body = SystemHotkeysDialogBodyView;
    rsx! {
        if open {
            WarcraftDialog::<SystemHotkeysDialogBodyView, Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(SystemHotkeysDialog);
