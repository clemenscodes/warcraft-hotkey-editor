use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::services::customkeys::upload_status::UploadStatus;

/// What the templates dialog needs: the loaded keys to overwrite when a template
/// is applied, the upload status to update, and the open signal.
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub templates_dialog_open: Signal<bool>,
}
