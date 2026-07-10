use super::view::TileOverrideAltPickerView;
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

impl From<&TileOverrideAltPickerView> for TileOverrideAltPickerProps {
    fn from(view: &TileOverrideAltPickerView) -> Self {
        let TileOverrideAltPickerView {
            object_id,
            display_name,
            picker_slots,
            alt_position_picker_open,
        } = view.clone();
        Self {
            object_id,
            display_name,
            picker_slots,
            alt_position_picker_open,
        }
    }
}

impl ddd::Props for TileOverrideAltPickerProps {
    type View = TileOverrideAltPickerView;
}
