mod hooks;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::{
    SystemSlot, SystemSlotProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use dioxus::prelude::*;
use hooks::use_slot_button;
pub use props::SlotButtonProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SlotButton);

/// A big WC3 slot for the hero-selection and control-group layouts: a framed cell
/// showing a caption and its bound key, edited on click via the system key picker.
/// The host owns the focusable button and the cell's size; the framed `SystemSlot`
/// draws the cell.
#[component]
pub fn SlotButton(props: SlotButtonProps) -> Element {
    let model = use_slot_button(&props);
    let slot = SystemSlotProps::from(&model);
    let picker = SystemKeyPickerDialogProps::from(&model);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            tabindex: "0",
            onclick: model.on_click,
            SystemSlot { ..slot }
        }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
