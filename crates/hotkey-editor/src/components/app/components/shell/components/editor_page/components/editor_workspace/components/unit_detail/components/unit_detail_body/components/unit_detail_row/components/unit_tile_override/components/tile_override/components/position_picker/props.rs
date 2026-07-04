use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridLayout;
use warcraft_keybinds::GridSlotId;

use super::alt_position_picker_explainer::AltPositionPickerExplainerProps;
use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};

/// The off-state position picker: a modal command grid where the off-state button can
/// be dragged to a new cell.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerProps {
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

impl From<&AltPositionPickerProps> for AltPositionPickerExplainerProps {
    fn from(_props: &AltPositionPickerProps) -> Self {
        let text = String::from(
            "Drag the off-state button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact.",
        );
        Self { text }
    }
}
