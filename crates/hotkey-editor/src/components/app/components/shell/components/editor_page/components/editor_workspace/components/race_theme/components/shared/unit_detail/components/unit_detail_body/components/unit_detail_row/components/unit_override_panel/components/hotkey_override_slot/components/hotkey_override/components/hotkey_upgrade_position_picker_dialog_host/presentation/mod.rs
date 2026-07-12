use super::model::HotkeyUpgradePositionPickerDialogHostModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
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

/// The host's seam: lock body scroll while the picker is open, then shape the open dialog
/// — or `None` when the picker is closed or the ability has no upgraded form. The body
/// region builds its own grid config from context; the host only provides the title and
/// the body's inputs.
pub(super) fn use_hotkey_upgrade_position_picker_dialog_host(
    props: &HotkeyUpgradePositionPickerDialogHostModel,
) -> Option<OpenHotkeyUpgradePositionPickerDialog> {
    let open_signal = props.hotkey_upgrade_position_picker_open;
    use_body_scroll_lock(open_signal);
    let is_open = open_signal();
    if !is_open {
        return None;
    }
    let upgrade_unit_id = props.upgrade_unit_id?;
    let title = format!("Position: {} (upgraded)", props.display_name);
    let picker_slots = props.picker_slots.clone();
    let mut close_signal = open_signal;
    let on_open_change = Callback::new(move |is_open: bool| {
        if !is_open {
            close_signal.set(false);
        }
    });
    let dialog = OpenHotkeyUpgradePositionPickerDialog {
        title,
        upgrade_unit_id,
        picker_slots,
        on_open_change,
    };
    Some(dialog)
}
