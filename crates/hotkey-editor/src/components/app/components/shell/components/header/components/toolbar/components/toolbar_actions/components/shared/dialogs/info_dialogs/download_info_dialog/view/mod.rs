use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DownloadInfoDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for DownloadInfoDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DownloadInfoDialogView {}
