pub mod components;
mod hooks;
mod logic;
mod props;

use components::key_chip::{KeyChip, KeyChipProps};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use dioxus::prelude::*;
use hooks::use_key_capture;
pub use props::KeyCaptureProps;
use tw_macro::assert_component;
assert_component!(KeyCapture);

/// The connected host for a system-hotkey list row: it sources the row's resolved
/// binding through `use_key_capture`, renders the presentational `KeyChip`, and
/// mounts the system key picker beneath itself while editing.
#[component]
pub fn KeyCapture(props: KeyCaptureProps) -> Element {
    let model = use_key_capture(&props);
    let chip = KeyChipProps::from(&model);
    let picker = SystemKeyPickerDialogProps::from(&model);
    rsx! {
        KeyChip { ..chip }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
