use super::super::inventory_cell::InventoryCellProps;
use dioxus::prelude::*;

/// One grid position: the finished cell props when the slot is filled, or `None`
/// for an empty position (which renders the placeholder).
#[derive(Props, Clone, PartialEq)]
pub struct InventorySlotProps {
    pub cell: Option<InventoryCellProps>,
}
