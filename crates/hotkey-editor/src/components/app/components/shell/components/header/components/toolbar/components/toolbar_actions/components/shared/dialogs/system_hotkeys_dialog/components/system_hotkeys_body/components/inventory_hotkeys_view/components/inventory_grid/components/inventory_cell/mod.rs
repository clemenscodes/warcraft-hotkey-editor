mod hooks;
mod logic;
mod props;
mod state;
mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_key::{
    SystemSlotKey, SystemSlotKeyProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::{
    SystemSlotLabel, SystemSlotLabelProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use dioxus::prelude::*;
use hooks::use_inventory_cell;
pub use props::InventoryCellProps;
assert_component!(InventoryCell);

/// A single draggable inventory slot: shows its caption and bound key, edits on
/// click via the system key picker, and swaps with another slot on drag.
#[component]
pub fn InventoryCell(props: InventoryCellProps) -> Element {
    let model = use_inventory_cell(&props);
    let label = SystemSlotLabelProps::from(&model);
    let key = SystemSlotKeyProps::from(&model);
    let picker = SystemKeyPickerDialogProps::from(&model);
    let class = style::class(model.state);
    rsx! {
        div {
            class,
            "data-inventory-slot": model.slot_id,
            "data-dragging": model.dragging_attr,
            tabindex: "0",
            "data-tooltip": model.conflict_title,
            onpointerdown: model.on_pointerdown,
            onpointermove: model.on_pointermove,
            onpointerup: model.on_pointerup,
            onpointercancel: model.on_pointercancel,
            onclick: model.on_click,
            SystemSlotLabel { ..label }
            SystemSlotKey { ..key }
        }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
