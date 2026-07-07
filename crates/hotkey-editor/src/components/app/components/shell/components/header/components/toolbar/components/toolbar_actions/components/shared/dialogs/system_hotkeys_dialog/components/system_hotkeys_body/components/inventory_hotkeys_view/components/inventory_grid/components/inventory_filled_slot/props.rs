use super::super::super::{InventoryDragFollower, InventoryDragSource};
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// One inventory slot: its index and section, plus the shared editing/drag signals
/// the grid coordinates. Its binding + conflicts come from the CustomKeys query, so
/// it holds neither the loaded keys nor a prebuilt binding map.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryFilledSlotProps {
    pub slot_index: usize,
    pub section_id: WarcraftObjectId,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub dragging_source: Signal<Option<InventoryDragSource>>,
    pub drop_target: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
