pub mod components;
mod model;
mod presentation;
pub mod state;
mod view;

pub use view::SystemHotkeysDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::system_hotkeys_dialog_body::SystemHotkeysDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::SystemHotkeysDialogModel;
use presentation::SystemHotkeysDialogShell;
use presentation::use_system_hotkeys_dialog;
use tw_macro::assert_component;

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
            WarcraftDialog::<SystemHotkeysDialogBodyView,Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(SystemHotkeysDialog);
