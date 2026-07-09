mod hooks;
mod logic;
mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::{
    SystemSlot, SystemSlotProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use dioxus::prelude::*;
use hooks::use_inventory_filled_slot;
pub use props::InventoryFilledSlotProps;
use style::CLASS;
assert_component!(InventoryFilledSlot);

/// A single draggable inventory slot: shows its caption and bound key, edits on
/// click via the system key picker, and swaps with another slot on drag. The host
/// owns the focusable, draggable outer cell and its size; the framed `SystemSlot`
/// draws the cell.
#[component]
pub fn InventoryFilledSlot(props: InventoryFilledSlotProps) -> Element {
    let model = use_inventory_filled_slot(&props);
    let slot = SystemSlotProps::from(&model);
    let picker = SystemKeyPickerDialogProps::from(&model);
    rsx! {
        div {
            class: CLASS,
            "data-inventory-slot": model.section_id.value(),
            tabindex: "0",
            onpointerdown: model.on_pointerdown,
            onpointermove: model.on_pointermove,
            onpointerup: model.on_pointerup,
            onpointercancel: model.on_pointercancel,
            onclick: model.on_click,
            SystemSlot { ..slot }
        }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
