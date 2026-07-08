mod hooks;
mod logic;
mod props;
mod state;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_key::{
    SystemSlotKey, SystemSlotKeyProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::{
    SystemSlotLabel, SystemSlotLabelProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use crate::components::app::components::shell::components::shared::tooltip::{
    Tooltip, TooltipProps,
};
use dioxus::prelude::*;
use hooks::use_slot_button;
pub use props::SlotButtonProps;
pub use state::SlotButtonState;
assert_component!(SlotButton);

/// A big WC3 slot for the hero-selection and control-group layouts: a framed cell
/// showing a caption and its bound key, edited on click via the system key picker.
#[component]
pub fn SlotButton(props: SlotButtonProps) -> Element {
    let model = use_slot_button(&props);
    let label = SystemSlotLabelProps::from(&model);
    let key = SystemSlotKeyProps::from(&model);
    let tooltip = TooltipProps::from(&model);
    let picker = SystemKeyPickerDialogProps::from(&model);
    let class = style::class(model.state);
    rsx! {
        button {
            class,
            r#type: "button",
            tabindex: "0",
            "data-compact": model.compact_attr,
            onclick: model.on_click,
            SystemSlotLabel { ..label }
            SystemSlotKey { ..key }
            Tooltip { ..tooltip }
        }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
