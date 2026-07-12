use super::model::HotkeyAltPositionPickerDialogHostModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;

/// The open off-state position picker's shaped data: the dialog title and the change
/// handler mirroring the headless dialog's own close (escape, outside click) back to the
/// open signal, which it clears. No `Signal<T>` crosses here — the open state rides out as
/// the plain `bool` the host passes to `WarcraftDialog`.
pub(super) struct OpenHotkeyAltPositionPickerDialog {
    pub(super) title: String,
    pub(super) on_open_change: Callback<bool>,
}

/// The host's seam: lock body scroll while the picker is open, then — when the open signal
/// is set — shape the open dialog (its title and close handler), or `None` when it is
/// closed so the host early-returns an empty mount.
pub(super) fn use_hotkey_alt_position_picker_dialog_host(
    props: &HotkeyAltPositionPickerDialogHostModel,
) -> Option<OpenHotkeyAltPositionPickerDialog> {
    let open_signal = props.hotkey_alt_position_picker_open;
    use_body_scroll_lock(open_signal);
    let is_open = *open_signal.read();
    if !is_open {
        return None;
    }
    let title = format!("Position: {}", props.display_name);
    let mut close_signal = open_signal;
    let on_open_change = Callback::new(move |is_open: bool| {
        if !is_open {
            close_signal.set(false);
        }
    });
    let dialog = OpenHotkeyAltPositionPickerDialog {
        title,
        on_open_change,
    };
    Some(dialog)
}
