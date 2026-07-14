use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

#[derive(Clone, PartialEq)]
pub struct HotkeyUpgradePositionPickerDialogView {
    pub upgrade_unit_id: Option<WarcraftObjectId>,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for HotkeyUpgradePositionPickerDialogView {}
