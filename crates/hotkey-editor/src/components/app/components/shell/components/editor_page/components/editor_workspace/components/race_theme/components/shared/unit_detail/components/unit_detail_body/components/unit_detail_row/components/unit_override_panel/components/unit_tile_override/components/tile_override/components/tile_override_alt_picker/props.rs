use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// Guards the off-state position picker so it is mounted only while it is open.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideAltPickerProps {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub alt_position_picker_open: Signal<bool>,
}
