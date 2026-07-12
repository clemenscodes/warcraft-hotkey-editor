use super::view::HotkeyUpgradePositionPickerDialogView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The upgraded-form position picker: a modal command grid where the upgraded-form
/// button can be dragged to a new cell. The shared editor signals the grid needs are
/// sourced from context by the component's hook.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUpgradePositionPickerDialogModel {
    pub upgrade_unit_id: WarcraftObjectId,
    pub display_name: String,
    pub picker_slots: Rc<[GridSlotId]>,
    pub hotkey_upgrade_position_picker_open: Signal<bool>,
}

impl From<&HotkeyUpgradePositionPickerDialogView> for HotkeyUpgradePositionPickerDialogModel {
    fn from(view: &HotkeyUpgradePositionPickerDialogView) -> Self {
        let HotkeyUpgradePositionPickerDialogView {
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

impl ddd::Model for HotkeyUpgradePositionPickerDialogModel {
    type View = HotkeyUpgradePositionPickerDialogView;
}
