use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// Reads the shared open signal from context — the one the toolbar buttons flip. The
/// dialog's editors read and write the document through the CustomKeys service, so
/// the host threads no loaded keys.
pub(super) fn use_system_hotkeys_dialog_host() -> Signal<bool> {
    let overlay = use_overlay_state();
    overlay.system_hotkeys_open()
}
