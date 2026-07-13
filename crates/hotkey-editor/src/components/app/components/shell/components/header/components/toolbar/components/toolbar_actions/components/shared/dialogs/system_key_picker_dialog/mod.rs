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

/// Assigns a system or menu hotkey from a full on-screen keyboard (including keys the
/// letter picker does not offer, like function keys and the numpad). The composed hook
/// shapes the title and board, the shell struct names the open flag and board region
/// values, and this renders the reusable `WarcraftDialog` directly — handing it the
/// isolated keyboard body region. The title/close header chrome and the content box are
/// the dialog primitive's; focus and the keyboard listener belong to the picker, so this
/// component wires neither.
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
            WarcraftDialog::<SystemKeyPickerDialogBodyView, Empty> {
                title,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(SystemKeyPickerDialog);
