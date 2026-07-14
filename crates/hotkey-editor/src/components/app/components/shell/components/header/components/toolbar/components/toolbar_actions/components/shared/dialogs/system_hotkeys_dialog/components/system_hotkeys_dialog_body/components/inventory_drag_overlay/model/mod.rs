use super::view::InventoryDragOverlayView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InventoryDragOverlayModel {
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}

impl From<&InventoryDragOverlayView> for InventoryDragOverlayModel {
    fn from(view: &InventoryDragOverlayView) -> Self {
        let InventoryDragOverlayView { drag_follower } = view.clone();
        Self { drag_follower }
    }
}

impl ddd::Model for InventoryDragOverlayModel {
    type View = InventoryDragOverlayView;
}
