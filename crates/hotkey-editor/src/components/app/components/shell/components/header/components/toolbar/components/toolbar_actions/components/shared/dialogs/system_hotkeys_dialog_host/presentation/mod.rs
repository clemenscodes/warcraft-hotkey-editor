use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The system-hotkeys dialog host's shaped wiring: whether the editor is open and the
/// change handler mirroring the headless dialog's own close (escape, outside click) back
/// to the shared signal.
pub(super) struct SystemHotkeysDialogHostModel {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the shared open signal from context — the one the toolbar buttons flip — and
/// shapes it into a signal-free open value plus a change handler. The dialog's editors
/// read and write the document through the CustomKeys service, so the host threads no
/// loaded keys.
pub(super) fn use_system_hotkeys_dialog_host() -> SystemHotkeysDialogHostModel {
    let overlay = use_overlay_state();
    let dialog_open = overlay.system_hotkeys_open();
    let open = *dialog_open.read();
    let mut change_open = dialog_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    SystemHotkeysDialogHostModel {
        open,
        on_open_change,
    }
}
