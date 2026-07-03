use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The upload button owns no state of its own beyond the info dialog; it writes the
/// imported keys and the upload status back to the app.
#[derive(Props, Clone, PartialEq)]
pub struct UploadButtonProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
}
