mod model;
mod presentation;
mod view;

pub use view::InventoryFilledSlotView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::SystemSlot;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;
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
    let model = use_inventory_filled_slot(&props);
    let state = model.state;
    let slot_label = model.slot_label.clone();
    let key_label = model.key_label.clone();
    let conflict = model.is_conflict;
    let tooltip_text = model.conflict_title.clone();
    let tooltip_placement = TooltipPlacement::Above;
    let dragging = model.dragging;
    let is_editing = model.is_editing;
    let title = String::from("Pick a hotkey");
    let current_code = model.current_code;
    let conflicts = model.picker_conflicts.clone();
    let open = true;
    let on_pick = model.on_pick;
    let on_close = model.on_close;
    let on_pointerdown = model.on_pointerdown;
    let on_pointermove = model.on_pointermove;
    let on_pointerup = model.on_pointerup;
    let on_pointercancel = model.on_pointercancel;
    let on_click = model.on_click;
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
