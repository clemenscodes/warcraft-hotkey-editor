mod browser_event;
pub mod components;
mod data;
mod hooks;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use crate::assert_component;
use components::system_key_picker_board::SystemKeyPickerBoard;
use hooks::use_system_key_picker;

pub use props::SystemKeyPickerDialogProps;

assert_component!(SystemKeyPickerDialog);

/// Assigns a system or menu hotkey from a full on-screen keyboard (including keys
/// the shared `KeyPicker` does not offer, like function keys and the numpad). A
/// variant of the `Dialog` base: the hook shapes the open signal and the board,
/// and the body composes the shell with the keyboard.
#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogProps) -> Element {
    let model = use_system_key_picker(&props);
    let title = props.title.clone();
    rsx! {
        Dialog {
            open: model.open,
            title,
            SystemKeyPickerBoard { ..model.board }
        }
    }
}
