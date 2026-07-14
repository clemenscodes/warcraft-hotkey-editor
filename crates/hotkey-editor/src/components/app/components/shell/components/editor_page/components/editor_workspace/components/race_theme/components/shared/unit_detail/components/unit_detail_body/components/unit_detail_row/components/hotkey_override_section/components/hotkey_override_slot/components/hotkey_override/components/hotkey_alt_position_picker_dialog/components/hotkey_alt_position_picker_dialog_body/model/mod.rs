use super::view::HotkeyAltPositionPickerDialogBodyView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The off-state position picker body's inputs: the object being edited and its slot set.
/// The shared editor signals the embedded command grid needs are sourced from context by
/// the component's presentation, so only the picker's own identity is a prop.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyAltPositionPickerDialogBodyModel {
    pub object_id: WarcraftObjectId,
    pub picker_slots: Rc<[GridSlotId]>,
}

impl From<&HotkeyAltPositionPickerDialogBodyView> for HotkeyAltPositionPickerDialogBodyModel {
    fn from(view: &HotkeyAltPositionPickerDialogBodyView) -> Self {
        let HotkeyAltPositionPickerDialogBodyView {
            object_id,
            picker_slots,
        } = view.clone();
        Self {
            object_id,
            picker_slots,
        }
    }
}

impl ddd::Model for HotkeyAltPositionPickerDialogBodyModel {
    type View = HotkeyAltPositionPickerDialogBodyView;
}
