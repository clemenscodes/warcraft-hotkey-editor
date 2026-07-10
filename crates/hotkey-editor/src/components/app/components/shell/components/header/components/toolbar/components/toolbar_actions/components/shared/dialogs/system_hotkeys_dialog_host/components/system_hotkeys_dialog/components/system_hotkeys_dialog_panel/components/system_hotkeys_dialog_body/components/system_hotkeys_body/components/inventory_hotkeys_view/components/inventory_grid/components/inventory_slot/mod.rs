pub mod components;
mod props;
mod view;

pub use view::InventorySlotView;

use components::inventory_empty_slot::InventoryEmptySlot;
use components::inventory_filled_slot::InventoryFilledSlot;
use dioxus::prelude::*;
use props::InventorySlotProps;
use tw_macro::assert_component;

/// One inventory grid position: a filled, editable cell, or the empty placeholder.
/// A pure dispatcher, so it carries no class of its own.
#[component]
pub fn InventorySlot(props: InventorySlotProps) -> Element {
    let Some(filled) = props.filled else {
        return rsx! {
            InventoryEmptySlot {}
        };
    };
    let slot_index = filled.slot_index;
    let section_id = filled.section_id;
    let dragging_source = filled.dragging_source;
    let drop_target = filled.drop_target;
    let drag_follower = filled.drag_follower;
    rsx! {
        InventoryFilledSlot {
            slot_index,
            section_id,
            dragging_source,
            drop_target,
            drag_follower,
        }
    }
}

assert_component!(InventorySlot);
