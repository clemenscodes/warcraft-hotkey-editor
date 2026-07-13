use super::view::DownloadInfoDialogView;
use dioxus::prelude::*;

/// What the download dialog needs: the open value it drives and the change handler
/// mirroring the headless dialog's own close. The dialog reads the live document from
/// the CustomKeys service itself, so no confirm handler is threaded in.
#[derive(Props, Clone, PartialEq)]
pub struct DownloadInfoDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&DownloadInfoDialogView> for DownloadInfoDialogModel {
    fn from(view: &DownloadInfoDialogView) -> Self {
        let DownloadInfoDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for DownloadInfoDialogModel {
    type View = DownloadInfoDialogView;
}
