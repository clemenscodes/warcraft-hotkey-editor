use crate::services::customkeys::context::{use_loaded_keys, use_upload_status};
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The templates dialog host's shaped domain data: the loaded document a template
/// overwrites, the upload status it reflects, and the shared open signal the toolbar
/// buttons flip.
pub(super) struct TemplatesDialogHostModel {
    pub(super) loaded_keys: Signal<Option<CustomKeys>>,
    pub(super) upload_status: Signal<UploadStatus>,
    pub(super) open: Signal<bool>,
}

/// Shapes the templates dialog's inputs from context: the loaded document a template
/// overwrites, the upload status it reflects, and the shared open signal the toolbar
/// buttons flip.
pub(super) fn use_templates_dialog_host() -> TemplatesDialogHostModel {
    let loaded_keys = use_loaded_keys();
    let upload_status = use_upload_status();
    let overlay = use_overlay_state();
    let open = overlay.templates_dialog_open();
    TemplatesDialogHostModel {
        loaded_keys,
        upload_status,
        open,
    }
}
