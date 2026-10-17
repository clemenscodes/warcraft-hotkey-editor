use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UploadInfoDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for UploadInfoDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UploadInfoDialogView {}
