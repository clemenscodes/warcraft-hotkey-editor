mod model;
mod presentation;
mod view;

pub use view::InventoryFilledSlotView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::SystemSlot;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use dioxus::prelude::*;
use presentation::InventoryFilledSlotPresentation;
use presentation::use_inventory_filled_slot;
use model::InventoryFilledSlotModel;
use style::CLASS;
use tw_macro::assert_component;

/// A single draggable inventory slot: shows its caption and bound key, edits on
/// click via the system key picker, and swaps with another slot on drag. The host
/// owns the focusable, draggable outer cell and its size; the framed `SystemSlot`
/// draws the cell.
#[component]
pub fn InventoryFilledSlot(props: InventoryFilledSlotModel) -> Element {
    let InventoryFilledSlotPresentation {
        state,
        slot_label,
        key_label,
        conflict,
        tooltip_text,
        tooltip_placement,
        dragging,
        is_editing,
        title,
        current_code,
        conflicts,
        open,
        on_pick,
        on_close,
        on_pointerdown,
        on_pointermove,
        on_pointerup,
        on_pointercancel,
        on_click,
    } = use_inventory_filled_slot(&props);
    rsx! {
        div {
            class: CLASS,
            tabindex: "0",
            onpointerdown: on_pointerdown,
            onpointermove: on_pointermove,
            onpointerup: on_pointerup,
            onpointercancel: on_pointercancel,
            onclick: on_click,
            SystemSlot {
                state,
                slot_label,
                key_label,
                conflict,
                tooltip_text,
                tooltip_placement,
                dragging,
            }
        }
        if is_editing {
            SystemKeyPickerDialog {
                title,
                current_code,
                conflicts,
                open,
                on_pick,
                on_close,
            }
        }
    }
}

assert_component!(InventoryFilledSlot);
