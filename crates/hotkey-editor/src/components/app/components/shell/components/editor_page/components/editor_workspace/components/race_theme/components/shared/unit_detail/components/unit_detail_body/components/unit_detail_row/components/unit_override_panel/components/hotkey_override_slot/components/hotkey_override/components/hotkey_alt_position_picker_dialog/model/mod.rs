use super::view::HotkeyAltPositionPickerDialogView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// Guards the off-state position picker so it is mounted only while it is open.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyAltPositionPickerDialogModel {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&HotkeyAltPositionPickerDialogView> for HotkeyAltPositionPickerDialogModel {
    fn from(view: &HotkeyAltPositionPickerDialogView) -> Self {
        let HotkeyAltPositionPickerDialogView {
            object_id,
            display_name,
            picker_slots,
            open,
            on_open_change,
        } = view.clone();
        Self {
            object_id,
            display_name,
            picker_slots,
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for HotkeyAltPositionPickerDialogModel {
    type View = HotkeyAltPositionPickerDialogView;
}
