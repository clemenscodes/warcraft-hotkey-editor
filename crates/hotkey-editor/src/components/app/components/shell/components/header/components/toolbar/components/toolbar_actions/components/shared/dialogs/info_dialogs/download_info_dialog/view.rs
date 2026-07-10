use dioxus::prelude::*;

/// The published `View` contract mirroring [`DownloadInfoDialogProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DownloadInfoDialogView {
    pub open: Signal<bool>,
    pub on_confirm: EventHandler<()>,
}

impl ddd::View for DownloadInfoDialogView {}
