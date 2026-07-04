use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;

/// The overlay's only input: the current drag follower, or `None` when no
/// inventory slot is being dragged.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryDragOverlayProps {
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}
