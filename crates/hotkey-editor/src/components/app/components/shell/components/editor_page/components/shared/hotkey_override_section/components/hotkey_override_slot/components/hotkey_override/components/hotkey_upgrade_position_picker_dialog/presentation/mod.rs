use super::model::HotkeyUpgradePositionPickerDialogModel;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

pub(super) struct OpenHotkeyUpgradePositionPickerDialog {
    pub(super) title: String,
    pub(super) upgrade_unit_id: WarcraftObjectId,
    pub(super) picker_slots: Rc<[GridSlotId]>,
    pub(super) on_open_change: Callback<bool>,
}

pub(super) fn use_hotkey_upgrade_position_picker_dialog(
    props: &HotkeyUpgradePositionPickerDialogModel,
) -> Option<OpenHotkeyUpgradePositionPickerDialog> {
    if !props.open {
        return None;
    }
    let upgrade_unit_id = props.upgrade_unit_id?;
    let title = format!("Position: {} (upgraded)", props.display_name);
    let picker_slots = props.picker_slots.clone();
    let on_open_change = props.on_open_change;
    let dialog = OpenHotkeyUpgradePositionPickerDialog {
        title,
        upgrade_unit_id,
        picker_slots,
        on_open_change,
    };
    Some(dialog)
}
