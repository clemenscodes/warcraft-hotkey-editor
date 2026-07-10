use super::view::TemplatesDialogView;
use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// What the templates dialog needs: the loaded keys to overwrite when a template
/// is applied, the upload status to update, and the open signal.
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub open: Signal<bool>,
}

impl From<&TemplatesDialogView> for TemplatesDialogProps {
    fn from(view: &TemplatesDialogView) -> Self {
        let TemplatesDialogView {
            loaded_keys,
            upload_status,
            open,
        } = view.clone();
        Self {
            loaded_keys,
            upload_status,
            open,
        }
    }
}

impl ddd::Props for TemplatesDialogProps {
    type View = TemplatesDialogView;
}
