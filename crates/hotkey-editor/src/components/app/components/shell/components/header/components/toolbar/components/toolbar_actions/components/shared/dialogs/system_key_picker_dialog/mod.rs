pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::SystemKeyPickerDialogView;
mod state;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::system_key_picker_dialog_body::SystemKeyPickerDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::SystemKeyPickerDialogModel;
use presentation::SystemKeyPickerDialogShell;
use presentation::use_system_key_picker;
use tw_macro::assert_component;

#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogModel) -> Element {
    let model = use_system_key_picker(&props);
    let SystemKeyPickerDialogShell {
        open,
        on_open_change,
        title,
        columns,
        on_pick,
        board_on_close,
    } = SystemKeyPickerDialogShell::from(&model);
    let body = SystemKeyPickerDialogBodyView {
        columns,
        on_pick,
        on_close: board_on_close,
    };
    rsx! {
        if open {
            WarcraftDialog::<SystemKeyPickerDialogBodyView,Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(SystemKeyPickerDialog);
