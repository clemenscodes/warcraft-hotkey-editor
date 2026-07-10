use dioxus::prelude::*;

/// The published `View` contract mirroring [`UploadInfoDialogProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UploadInfoDialogView {
    pub open: Signal<bool>,
}

impl ddd::View for UploadInfoDialogView {}
