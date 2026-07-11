use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::{
    InventoryDragFollower, InventoryDragSource,
};
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// The published `View` contract mirroring [`InventoryFilledSlotModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InventoryFilledSlotView {
    pub slot_index: usize,
    pub section_id: WarcraftObjectId,
    pub dragging_source: Signal<Option<InventoryDragSource>>,
    pub drop_target: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}

impl ddd::View for InventoryFilledSlotView {}
