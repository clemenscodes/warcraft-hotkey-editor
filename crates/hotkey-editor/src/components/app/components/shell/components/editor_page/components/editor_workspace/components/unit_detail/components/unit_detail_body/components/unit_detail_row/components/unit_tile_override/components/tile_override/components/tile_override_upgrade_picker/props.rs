use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

use super::super::position_picker::UpgradePositionPickerProps;
use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};

/// Guards the upgraded-form position picker: it only exists when the ability has an
/// upgraded form, so its unit id is optional here.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideUpgradePickerProps {
    pub visible: bool,
    pub upgrade_unit_id: Option<WarcraftObjectId>,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub grid_layout: Signal<GridLayout>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub upgrade_position_picker_open: Signal<bool>,
}

impl From<&TileOverrideUpgradePickerProps> for UpgradePositionPickerProps {
    /// Only called after the body guards that `upgrade_unit_id` is present and the
    /// picker is visible, so the unwrap holds.
    fn from(props: &TileOverrideUpgradePickerProps) -> Self {
        let upgrade_unit_id = props
            .upgrade_unit_id
            .expect("guarded to Some before conversion");
        let display_name = props.display_name.clone();
        let picker_slots = props.picker_slots.clone();
        let loaded_keys = props.loaded_keys;
        let grid_layout = props.grid_layout;
        let dragging_slot = props.dragging_slot;
        let drop_target_tile = props.drop_target_tile;
        let drag_follower = props.drag_follower;
        let upgrade_position_picker_open = props.upgrade_position_picker_open;
        Self {
            upgrade_unit_id,
            display_name,
            picker_slots,
            loaded_keys,
            grid_layout,
            dragging_slot,
            drop_target_tile,
            drag_follower,
            upgrade_position_picker_open,
        }
    }
}
