use super::components::upload_info_actions::{UploadInfoActions, UploadInfoActionsProps};
use super::components::upload_info_content::UploadInfoContent;
use super::props::UploadInfoDialogProps;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&UploadInfoDialogProps> for DialogProps {
    fn from(props: &UploadInfoDialogProps) -> Self {
        let open = props.open;
        let title = String::from("Import CustomKeys.txt");
        let actions = UploadInfoActionsProps::from(props);
        let children = rsx! {
            UploadInfoContent {}
            UploadInfoActions { ..actions }
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
