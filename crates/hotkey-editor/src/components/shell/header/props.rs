use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, GridLayout};

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub grid_layout: Signal<GridLayout>,
}
