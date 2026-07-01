mod hooks;
mod props;
mod state;
mod style;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::system_slot_key::SystemSlotKey;
use crate::components::dialogs::system_hotkeys_dialog::components::system_slot_label::SystemSlotLabel;
use crate::components::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use dioxus::prelude::*;
use hooks::use_inventory_cell;
pub use props::InventoryCellProps;
assert_component!(InventoryCell);

/// A single draggable inventory slot: shows its caption and bound key, edits on
/// click via the system key picker, and swaps with another slot on drag.
#[component]
pub fn InventoryCell(props: InventoryCellProps) -> Element {
    let model = use_inventory_cell(&props);
    let class = style::class(model.state);
    let section_id = props.section_id.clone();
    rsx! {
        div {
            class,
            "data-inventory-slot": section_id,
            "data-dragging": model
                    .dragging_attr,
            tabindex: "0",
            "data-tooltip": model.conflict_title,
            "data-tooltip-placement": "above",
            onpointerdown: model.on_pointerdown,
            onpointermove: model.on_pointermove,
            onpointerup: model.on_pointerup,
            onpointercancel: model.on_pointercancel,
            onclick: model.on_click,
            SystemSlotLabel { text: model.slot_label, compact: false }
            SystemSlotKey {
                label: model.key_label,
                compact: false,
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
