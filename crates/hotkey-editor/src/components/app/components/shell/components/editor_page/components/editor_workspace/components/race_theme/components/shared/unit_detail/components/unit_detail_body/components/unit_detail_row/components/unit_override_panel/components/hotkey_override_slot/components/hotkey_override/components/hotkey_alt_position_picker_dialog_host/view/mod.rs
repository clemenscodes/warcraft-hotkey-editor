use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The published `View` contract mirroring [`HotkeyAltPositionPickerDialogHostModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyAltPositionPickerDialogHostView {
    pub object_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for HotkeyAltPositionPickerDialogHostView {}
