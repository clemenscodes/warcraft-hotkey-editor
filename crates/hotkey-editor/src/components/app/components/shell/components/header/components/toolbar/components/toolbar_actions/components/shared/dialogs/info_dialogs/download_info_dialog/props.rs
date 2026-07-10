use super::view::DownloadInfoDialogView;
use dioxus::prelude::*;

/// What the download dialog needs: the open signal it drives and the confirm
/// handler the parent uses to actually trigger the download.
#[derive(Props, Clone, PartialEq)]
pub struct DownloadInfoDialogProps {
    pub open: Signal<bool>,
    pub on_confirm: EventHandler<()>,
}

impl From<&DownloadInfoDialogView> for DownloadInfoDialogProps {
    fn from(view: &DownloadInfoDialogView) -> Self {
        let DownloadInfoDialogView { open, on_confirm } = view.clone();
        Self { open, on_confirm }
    }
}

impl ddd::Props for DownloadInfoDialogProps {
    type View = DownloadInfoDialogView;
}
