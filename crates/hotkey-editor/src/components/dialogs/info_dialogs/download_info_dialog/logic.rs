use super::data::{INTRO, PRIMARY_LABEL, TITLE, WARNING};
use super::props::DownloadInfoDialogProps;
use crate::components::dialogs::info_dialogs::info_dialog::InfoDialogConfig;
use dioxus::prelude::*;

impl From<&DownloadInfoDialogProps> for InfoDialogConfig {
    fn from(props: &DownloadInfoDialogProps) -> Self {
        let mut open = props.open;
        let on_confirm = props.on_confirm;
        let on_cancel = EventHandler::new(move |_event: MouseEvent| open.set(false));
        let on_primary = EventHandler::new(move |_event: MouseEvent| {
            open.set(false);
            on_confirm.call(());
        });
        let title = TITLE;
        let intro = INTRO;
        let warning = Some(WARNING);
        let primary_label = PRIMARY_LABEL;
        Self {
            open,
            title,
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}
