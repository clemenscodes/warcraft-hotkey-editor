use super::components::download_info_actions::{DownloadInfoActions, DownloadInfoActionsProps};
use super::components::download_info_content::DownloadInfoContent;
use super::props::DownloadInfoDialogProps;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&DownloadInfoDialogProps> for DialogProps {
    fn from(props: &DownloadInfoDialogProps) -> Self {
        let open = props.open;
        let title = String::from("Download CustomKeys.txt");
        let actions = DownloadInfoActionsProps::from(props);
        let children = rsx! {
            DownloadInfoContent {}
            DownloadInfoActions { ..actions }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
