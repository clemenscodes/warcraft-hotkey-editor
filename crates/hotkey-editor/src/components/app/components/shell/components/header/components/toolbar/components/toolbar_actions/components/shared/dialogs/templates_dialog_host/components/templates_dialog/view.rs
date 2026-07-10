use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

/// The published `View` contract mirroring [`TemplatesDialogProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplatesDialogView {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub open: Signal<bool>,
}

impl ddd::View for TemplatesDialogView {}
