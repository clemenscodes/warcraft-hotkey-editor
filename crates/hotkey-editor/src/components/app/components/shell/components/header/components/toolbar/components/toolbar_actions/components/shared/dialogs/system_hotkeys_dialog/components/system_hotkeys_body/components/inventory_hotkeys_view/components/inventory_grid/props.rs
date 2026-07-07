use super::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// What the inventory grid needs: the shared editing-section signal and the drag
/// follower its cells drive. Each cell resolves its own binding from the CustomKeys
/// query.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryGridProps {
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
