use super::view::HotkeyUpgradePositionPickerDialogView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUpgradePositionPickerDialogModel {
    pub upgrade_unit_id: Option<WarcraftObjectId>,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&HotkeyUpgradePositionPickerDialogView> for HotkeyUpgradePositionPickerDialogModel {
    fn from(view: &HotkeyUpgradePositionPickerDialogView) -> Self {
        let HotkeyUpgradePositionPickerDialogView {
            upgrade_unit_id,
            display_name,
            picker_slots,
            open,
            on_open_change,
        } = view.clone();
        Self {
            upgrade_unit_id,
            display_name,
            picker_slots,
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for HotkeyUpgradePositionPickerDialogModel {
    type View = HotkeyUpgradePositionPickerDialogView;
}
