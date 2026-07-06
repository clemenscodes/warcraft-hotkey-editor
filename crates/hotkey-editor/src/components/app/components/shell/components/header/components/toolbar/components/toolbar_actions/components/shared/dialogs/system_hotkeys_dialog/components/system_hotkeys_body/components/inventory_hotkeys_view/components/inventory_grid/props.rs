use super::InventoryDragFollower;
use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, WarcraftObjectId};

/// What the inventory grid needs: the loaded keys its cells edit, the shared
/// editing-section signal, and the drag follower the cells drive.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryGridProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
