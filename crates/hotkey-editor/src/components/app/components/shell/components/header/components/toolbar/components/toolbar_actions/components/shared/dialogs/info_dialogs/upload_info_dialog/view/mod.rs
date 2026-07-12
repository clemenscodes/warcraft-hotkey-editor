use dioxus::prelude::*;

/// The published `View` contract mirroring [`UploadInfoDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UploadInfoDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for UploadInfoDialogView {}
