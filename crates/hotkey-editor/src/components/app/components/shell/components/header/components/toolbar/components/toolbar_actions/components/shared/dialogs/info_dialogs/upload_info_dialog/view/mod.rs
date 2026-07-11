use dioxus::prelude::*;

/// The published `View` contract mirroring [`UploadInfoDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UploadInfoDialogView {
    pub open: Signal<bool>,
}

impl ddd::View for UploadInfoDialogView {}
