use super::view::AltPositionPickerView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The off-state position picker: a modal command grid where the off-state button can
/// be dragged to a new cell. The shared editor signals the grid needs are sourced from
/// context by the component's hook, so only the picker's own identity is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerProps {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub alt_position_picker_open: Signal<bool>,
}

impl From<&AltPositionPickerView> for AltPositionPickerProps {
    fn from(view: &AltPositionPickerView) -> Self {
        let AltPositionPickerView {
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

impl ddd::Props for AltPositionPickerProps {
    type View = AltPositionPickerView;
}
