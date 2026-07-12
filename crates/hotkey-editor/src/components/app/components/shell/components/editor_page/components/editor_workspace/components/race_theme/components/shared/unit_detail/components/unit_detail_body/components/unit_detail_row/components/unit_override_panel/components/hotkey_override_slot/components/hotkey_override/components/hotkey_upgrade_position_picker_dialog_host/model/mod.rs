use super::view::HotkeyUpgradePositionPickerDialogHostView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// Guards the upgraded-form position picker: it only exists when the ability has an
/// upgraded form, so its unit id is optional here.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUpgradePositionPickerDialogHostModel {
    pub upgrade_unit_id: Option<WarcraftObjectId>,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub hotkey_upgrade_position_picker_open: Signal<bool>,
}

impl From<&HotkeyUpgradePositionPickerDialogHostView>
    for HotkeyUpgradePositionPickerDialogHostModel
{
    fn from(view: &HotkeyUpgradePositionPickerDialogHostView) -> Self {
        let HotkeyUpgradePositionPickerDialogHostView {
            upgrade_unit_id,
            display_name,
            picker_slots,
            hotkey_upgrade_position_picker_open,
        } = view.clone();
        Self {
            upgrade_unit_id,
            display_name,
            picker_slots,
            hotkey_upgrade_position_picker_open,
        }
    }
}

impl ddd::Model for HotkeyUpgradePositionPickerDialogHostModel {
    type View = HotkeyUpgradePositionPickerDialogHostView;
}
