mod browser_event;
pub mod components;
mod data;
mod hooks;
mod logic;
mod props;
mod state;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_system_key_picker;
pub use props::SystemKeyPickerDialogProps;

/// Assigns a system or menu hotkey from a full on-screen keyboard (including keys
/// the shared `KeyPicker` does not offer, like function keys and the numpad). A
/// variant of the `Dialog` base: the hook shapes the open signal and the board,
/// and the body composes the shell with the keyboard.
#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogProps) -> Element {
    let model = use_system_key_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
