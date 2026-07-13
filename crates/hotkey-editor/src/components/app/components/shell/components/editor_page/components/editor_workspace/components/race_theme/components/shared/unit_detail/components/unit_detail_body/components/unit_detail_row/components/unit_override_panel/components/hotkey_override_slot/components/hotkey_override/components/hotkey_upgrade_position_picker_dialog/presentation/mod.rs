use super::model::HotkeyUpgradePositionPickerDialogModel;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;

/// The open upgraded-form position picker's shaped data: the dialog title, the
/// upgraded-form unit id and command-card slots the body builds its grid from, and the
/// change handler mirroring the headless dialog's own close (escape, outside click) back
/// to the trigger's open signal, which it clears.
pub(super) struct OpenHotkeyUpgradePositionPickerDialog {
    pub(super) title: String,
    pub(super) upgrade_unit_id: WarcraftObjectId,
    pub(super) picker_slots: Rc<[GridSlotId]>,
    pub(super) on_open_change: Callback<bool>,
}

/// The host's seam: shape the open dialog when the picker is open — or `None` when it is
/// closed or the ability has no upgraded form. Body scroll is locked once by
/// `WarcraftDialog`. The body region builds its own grid config from context; the host
/// only provides the title and the body's inputs.
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
