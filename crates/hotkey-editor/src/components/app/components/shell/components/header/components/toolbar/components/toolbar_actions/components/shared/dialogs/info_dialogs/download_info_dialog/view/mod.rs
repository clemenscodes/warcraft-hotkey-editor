use dioxus::prelude::*;

/// The published `View` contract mirroring [`DownloadInfoDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DownloadInfoDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for DownloadInfoDialogView {}
