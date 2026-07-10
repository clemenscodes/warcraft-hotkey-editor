use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`InventoryDragOverlayProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InventoryDragOverlayView {
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}

impl ddd::View for InventoryDragOverlayView {}
