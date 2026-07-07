mod props;

use super::inventory_empty_slot::InventoryEmptySlot;
use super::inventory_filled_slot::InventoryFilledSlot;
use dioxus::prelude::*;
pub use props::InventorySlotProps;

/// One inventory grid position: a filled, editable cell, or the empty placeholder.
/// A pure dispatcher, so it carries no class of its own.
#[component]
pub fn InventorySlot(props: InventorySlotProps) -> Element {
    let Some(cell) = props.cell else {
        return rsx! {
            InventoryEmptySlot {}
        };
    };
    rsx! {
        InventoryFilledSlot { ..cell }
    }
}
