pub mod components;
mod data;
mod hooks;
mod logic;
mod props;
mod state;
mod style;

use components::system_key_picker_dialog_body::SystemKeyPickerDialogBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_system_key_picker;
use logic::SystemKeyPickerDialogShell;
pub use props::SystemKeyPickerDialogProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(SystemKeyPickerDialog);

/// Assigns a system or menu hotkey from a full on-screen keyboard (including keys the
/// letter picker does not offer, like function keys and the numpad). It owns its own
/// dialog shell: the composed hook shapes the title and board, the shell struct names
/// the header and scroll body, and this places them inside the backdrop and bordered
/// box. Focus and the keyboard listener belong to the picker, not the dialog, so this
/// component wires neither.
#[component]
pub fn SystemKeyPickerDialog(props: SystemKeyPickerDialogProps) -> Element {
    let model = use_system_key_picker(&props);
    use_body_scroll_lock(model.open);
    let SystemKeyPickerDialogShell {
        open,
        on_open_change,
        header,
        body,
    } = SystemKeyPickerDialogShell::from(&model);
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                SystemKeyPickerDialogBody { ..body }
            }
        }
    }
}
