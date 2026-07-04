use super::super::system_hotkeys_dialog::SystemHotkeysDialogProps;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::overlay_state::context::use_overlay_state;

/// Shapes the system-hotkeys dialog's props from context: the loaded document its
/// editors read and write, and the shared open signal the toolbar buttons flip.
pub(super) fn use_system_hotkeys_dialog_host() -> SystemHotkeysDialogProps {
    let loaded_keys = use_loaded_keys();
    let overlay = use_overlay_state();
    let system_hotkeys_open = overlay.system_hotkeys_open;
    SystemHotkeysDialogProps {
        loaded_keys,
        system_hotkeys_open,
    }
}
