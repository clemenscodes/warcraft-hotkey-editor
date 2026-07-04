use super::super::shared::dialogs::templates_dialog::TemplatesDialogProps;
use crate::services::customkeys::context::{use_loaded_keys, use_upload_status};
use crate::services::overlay_state::context::use_overlay_state;

/// Shapes the templates dialog's props from context: the loaded document a template
/// overwrites, the upload status it reflects, and the shared open signal the toolbar
/// buttons flip.
pub(super) fn use_templates_dialog_host() -> TemplatesDialogProps {
    let loaded_keys = use_loaded_keys();
    let upload_status = use_upload_status();
    let overlay = use_overlay_state();
    let open = overlay.templates_dialog_open;
    TemplatesDialogProps {
        loaded_keys,
        upload_status,
        open,
    }
}
