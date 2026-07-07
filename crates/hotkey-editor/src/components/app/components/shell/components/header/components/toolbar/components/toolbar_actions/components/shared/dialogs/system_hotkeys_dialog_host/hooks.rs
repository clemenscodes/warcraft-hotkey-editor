use super::super::system_hotkeys_dialog::SystemHotkeysDialogProps;
use crate::services::overlay_state::context::use_overlay_state;

/// Shapes the system-hotkeys dialog's props from context: the shared open signal the
/// toolbar buttons flip. The dialog's editors read and write the document through
/// the CustomKeys service, so the host threads no loaded keys.
pub(super) fn use_system_hotkeys_dialog_host() -> SystemHotkeysDialogProps {
    let overlay = use_overlay_state();
    let system_hotkeys_open = overlay.system_hotkeys_open;
    SystemHotkeysDialogProps {
        system_hotkeys_open,
    }
}
