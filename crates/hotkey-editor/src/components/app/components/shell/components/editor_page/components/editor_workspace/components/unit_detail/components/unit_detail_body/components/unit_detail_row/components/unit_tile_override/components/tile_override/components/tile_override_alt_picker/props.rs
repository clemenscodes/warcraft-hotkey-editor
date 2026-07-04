use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

use super::super::position_picker::AltPositionPickerProps;
use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};

/// Guards the off-state position picker so it is mounted only while it is open.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideAltPickerProps {
    pub visible: bool,
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub alt_position_picker_open: Signal<bool>,
}

impl From<&TileOverrideAltPickerProps> for AltPositionPickerProps {
    fn from(props: &TileOverrideAltPickerProps) -> Self {
        let object_id = props.object_id;
        let display_name = props.display_name.clone();
        let picker_slots = props.picker_slots.clone();
        let loaded_keys = props.loaded_keys;
        let grid_layout = props.grid_layout;
        let dragging_slot = props.dragging_slot;
        let drop_target_tile = props.drop_target_tile;
        let drag_follower = props.drag_follower;
        let alt_position_picker_open = props.alt_position_picker_open;
        Self {
            object_id,
            display_name,
            picker_slots,
            loaded_keys,
            grid_layout,
            dragging_slot,
            drop_target_tile,
            drag_follower,
            alt_position_picker_open,
        }
    }
}
