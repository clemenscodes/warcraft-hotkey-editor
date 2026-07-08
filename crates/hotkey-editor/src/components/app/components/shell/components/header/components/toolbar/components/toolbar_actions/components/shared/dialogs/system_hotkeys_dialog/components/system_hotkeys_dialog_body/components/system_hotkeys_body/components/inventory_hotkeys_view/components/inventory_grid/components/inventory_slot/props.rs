use super::components::inventory_filled_slot::InventoryFilledSlotProps;
use dioxus::prelude::*;

/// One grid position: the finished cell props when the slot is filled, or `None`
/// for an empty position (which renders the placeholder).
#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotProps {
    pub cell: Option<InventoryFilledSlotProps>,
}
