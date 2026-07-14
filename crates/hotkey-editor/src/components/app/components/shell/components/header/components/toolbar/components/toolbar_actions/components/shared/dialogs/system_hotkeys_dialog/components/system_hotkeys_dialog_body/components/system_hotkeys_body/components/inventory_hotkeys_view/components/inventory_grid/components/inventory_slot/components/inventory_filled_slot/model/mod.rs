use super::view::InventoryFilledSlotView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::{
    InventoryDragFollower, InventoryDragSource,
};
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct InventoryFilledSlotModel {
    pub slot_index: usize,
    pub section_id: WarcraftObjectId,
    pub dragging_source: Signal<Option<InventoryDragSource>>,
    pub drop_target: Signal<Option<WarcraftObjectId>>,
    pub drag_follower: Signal<Option<InventoryDragFollower>>,
}

impl From<&InventoryFilledSlotView> for InventoryFilledSlotModel {
    fn from(view: &InventoryFilledSlotView) -> Self {
        let InventoryFilledSlotView {
            slot_index,
            section_id,
            dragging_source,
            drop_target,
            drag_follower,
        } = view.clone();
        Self {
            slot_index,
            section_id,
            dragging_source,
            drop_target,
            drag_follower,
        }
    }
}

impl ddd::Model for InventoryFilledSlotModel {
    type View = InventoryFilledSlotView;
}
