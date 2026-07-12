use super::view::HotkeyAltPositionPickerDialogHostView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// Guards the off-state position picker so it is mounted only while it is open.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyAltPositionPickerDialogHostModel {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&HotkeyAltPositionPickerDialogHostView> for HotkeyAltPositionPickerDialogHostModel {
    fn from(view: &HotkeyAltPositionPickerDialogHostView) -> Self {
        let HotkeyAltPositionPickerDialogHostView {
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

impl ddd::Model for HotkeyAltPositionPickerDialogHostModel {
    type View = HotkeyAltPositionPickerDialogHostView;
}
