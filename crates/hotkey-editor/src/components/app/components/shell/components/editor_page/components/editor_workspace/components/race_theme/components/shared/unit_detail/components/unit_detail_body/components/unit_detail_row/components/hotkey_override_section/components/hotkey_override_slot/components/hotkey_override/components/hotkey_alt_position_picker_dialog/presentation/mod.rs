use super::model::HotkeyAltPositionPickerDialogModel;
use dioxus::prelude::*;

/// The open off-state position picker's shaped data: the dialog title and the change
/// handler mirroring the headless dialog's own close (escape, outside click) back to the
/// open signal, which it clears. No `Signal<T>` crosses here — the open state rides out as
/// the plain `bool` the host passes to `WarcraftDialog`.
pub(super) struct OpenHotkeyAltPositionPickerDialog {
    pub(super) title: String,
    pub(super) on_open_change: Callback<bool>,
}

/// The host's seam: when the open value is set, shape the open dialog (its title and close
/// handler), or `None` when it is closed so the host early-returns an empty mount. Body
/// scroll is locked once by `WarcraftDialog`.
pub(super) fn use_hotkey_alt_position_picker_dialog(
    props: &HotkeyAltPositionPickerDialogModel,
) -> Option<OpenHotkeyAltPositionPickerDialog> {
    if !props.open {
        return None;
    }
    let title = format!("Position: {}", props.display_name);
    let on_open_change = props.on_open_change;
    let dialog = OpenHotkeyAltPositionPickerDialog {
        title,
        on_open_change,
    };
    Some(dialog)
}
