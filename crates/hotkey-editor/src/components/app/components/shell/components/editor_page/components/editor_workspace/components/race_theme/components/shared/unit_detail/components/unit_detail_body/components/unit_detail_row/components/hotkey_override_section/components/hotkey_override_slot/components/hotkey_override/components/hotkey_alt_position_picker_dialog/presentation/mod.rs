use super::model::HotkeyAltPositionPickerDialogModel;
use dioxus::prelude::*;

pub(super) struct OpenHotkeyAltPositionPickerDialog {
    pub(super) title: String,
    pub(super) on_open_change: Callback<bool>,
}

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
