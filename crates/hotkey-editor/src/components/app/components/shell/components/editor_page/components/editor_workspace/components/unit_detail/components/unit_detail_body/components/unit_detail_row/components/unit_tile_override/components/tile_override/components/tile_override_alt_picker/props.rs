use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

use super::components::alt_position_picker::AltPositionPickerProps;

/// Guards the off-state position picker so it is mounted only while it is open.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideAltPickerProps {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub alt_position_picker_open: Signal<bool>,
}

impl From<&TileOverrideAltPickerProps> for AltPositionPickerProps {
    fn from(props: &TileOverrideAltPickerProps) -> Self {
        let object_id = props.object_id;
        let display_name = props.display_name.clone();
        let picker_slots = props.picker_slots.clone();
        let alt_position_picker_open = props.alt_position_picker_open;
        Self {
            object_id,
            display_name,
            picker_slots,
            alt_position_picker_open,
        }
    }
}
