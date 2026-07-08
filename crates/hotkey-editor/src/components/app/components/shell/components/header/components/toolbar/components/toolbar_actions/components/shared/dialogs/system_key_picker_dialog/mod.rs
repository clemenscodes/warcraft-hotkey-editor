mod data;
mod hooks;
mod logic;
mod props;
mod state;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_system_key_picker;
pub use props::SystemKeyPickerDialogProps;

/// Assigns a system or menu hotkey from a full on-screen keyboard (including keys the
/// letter picker does not offer, like function keys and the numpad). A `Dialog`
/// variant: it lays out the `KeyCode` keyboard columns and drops the shared,
/// dialog-agnostic [`KeyPickerBoardHost`] into the dialog body. Focus and the keyboard
/// listener belong to the picker, not the dialog, so this component wires neither.
use tw_macro::assert_component;
assert_component!(SystemKeyPickerDialog);
#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogProps) -> Element {
    let model = use_system_key_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
