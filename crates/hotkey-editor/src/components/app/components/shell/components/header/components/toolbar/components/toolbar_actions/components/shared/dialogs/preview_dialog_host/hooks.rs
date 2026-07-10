use super::components::preview_dialog::PreviewDialogProps;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::overlay_state::context::use_overlay_state;

/// Shapes the preview dialog's props from context: the loaded document to serialize
/// and the shared open signal the toolbar buttons flip.
pub(super) fn use_preview_dialog_host() -> PreviewDialogProps {
    let loaded_keys = use_loaded_keys();
    let overlay = use_overlay_state();
    let preview_open = overlay.preview_open();
    PreviewDialogProps {
        loaded_keys,
        preview_open,
    }
}
