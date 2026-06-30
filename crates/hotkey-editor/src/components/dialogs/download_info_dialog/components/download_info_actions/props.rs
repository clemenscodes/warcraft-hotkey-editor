use dioxus::prelude::*;

use crate::components::dialogs::download_info_dialog::DownloadInfoDialogProps;

/// The two action handlers, built from the dialog props. Cancel closes the
/// dialog; download closes it and forwards the confirm.
#[derive(Props, Clone, PartialEq)]
pub struct DownloadInfoActionsProps {
    pub on_cancel: EventHandler<MouseEvent>,
    pub on_download: EventHandler<MouseEvent>,
}

impl From<&DownloadInfoDialogProps> for DownloadInfoActionsProps {
    fn from(props: &DownloadInfoDialogProps) -> Self {
        let mut open = props.open;
        let on_confirm = props.on_confirm;
        let on_cancel = EventHandler::new(move |_event: MouseEvent| open.set(false));
        let on_download = EventHandler::new(move |_event: MouseEvent| {
            open.set(false);
            on_confirm.call(());
        });
        Self {
            on_cancel,
            on_download,
        }
    }
}
