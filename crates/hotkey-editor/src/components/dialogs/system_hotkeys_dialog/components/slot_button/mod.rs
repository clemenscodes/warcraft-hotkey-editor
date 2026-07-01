mod hooks;
mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::system_slot_key::SystemSlotKey;
use crate::components::dialogs::system_hotkeys_dialog::components::system_slot_label::SystemSlotLabel;
use crate::components::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use hooks::use_slot_button;

pub use props::SlotButtonProps;
pub use state::SlotButtonState;

assert_component!(SlotButton);

/// A big WC3 slot for the hero-selection and control-group layouts: a framed cell
/// showing a caption and its bound key, edited on click via the system key picker.
#[component]
pub fn SlotButton(props: SlotButtonProps) -> Element {
    let model = use_slot_button(&props);
    let class = style::class(model.state);
    let slot_label = props.slot_label.clone();
    rsx! {
        button {
            class,
            r#type: "button",
            tabindex: "0",
            "data-compact": model.compact_attr,
            "data-tooltip": model.conflict_title,
            onclick: model.on_click,
            SystemSlotLabel { text: slot_label, compact: model.compact }
            SystemSlotKey {
                label: model.key_label,
                compact: model.compact,
                conflict: model.is_conflict,
            }
        }
        if model.is_editing {
            SystemKeyPickerDialog {
                title: "Pick a hotkey",
                current_code: model.current_code,
                conflicts: model.picker_conflicts,
                open: true,
                on_pick: model.on_pick,
                on_close: model.on_close,
            }
        }
    }
}
