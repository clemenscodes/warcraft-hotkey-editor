use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::services::customkeys::upload_status::UploadStatus;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderToolbarProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
}
