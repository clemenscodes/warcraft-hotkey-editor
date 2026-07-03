use dioxus::prelude::*;

/// What the download dialog needs: the open signal it drives and the confirm
/// handler the parent uses to actually trigger the download.
#[derive(Props, Clone, PartialEq)]
pub struct DownloadInfoDialogProps {
    pub open: Signal<bool>,
    pub on_confirm: EventHandler<()>,
}
