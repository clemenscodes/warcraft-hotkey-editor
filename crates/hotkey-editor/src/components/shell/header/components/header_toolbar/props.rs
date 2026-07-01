use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderToolbarProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
}
