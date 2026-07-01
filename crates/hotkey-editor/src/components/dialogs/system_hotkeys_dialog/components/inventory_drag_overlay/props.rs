use dioxus::prelude::*;

use crate::components::dialogs::system_hotkeys_dialog::components::inventory_grid::InventoryDragFollower;

/// The overlay's only input: the current drag follower, or `None` when no
/// inventory slot is being dragged.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryDragOverlayProps {
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
