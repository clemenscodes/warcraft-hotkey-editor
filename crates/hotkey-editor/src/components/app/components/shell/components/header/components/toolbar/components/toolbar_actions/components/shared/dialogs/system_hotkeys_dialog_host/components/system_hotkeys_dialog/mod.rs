pub mod components;
mod model;
mod presentation;
mod view;

pub use view::SystemHotkeysDialogView;
pub mod state;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::system_hotkeys_dialog_body::SystemHotkeysDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::SystemHotkeysDialogModel;
use presentation::SystemHotkeysDialogShell;
use presentation::use_system_hotkeys_dialog;
use tw_macro::assert_component;

/// Edits Warcraft III's system and menu hotkeys. It renders the reusable `WarcraftDialog`
/// directly, handing it the isolated body region; the headless dialog gates itself on the
/// shell's open value and derives its own close from `on_open_change`. The hook holds the
/// UI signals and provides them to the subtree via context.
#[component]
pub fn SystemHotkeysDialog(props: SystemHotkeysDialogModel) -> Element {
    let model = use_system_hotkeys_dialog(&props);
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
