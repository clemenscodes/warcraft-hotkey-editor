use super::data::{INTRO, PRIMARY_LABEL, TITLE};
use super::props::UploadInfoDialogProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::InfoDialogConfig;
use crate::services::files::upload;
use dioxus::prelude::*;

impl From<&UploadInfoDialogProps> for InfoDialogConfig {
    fn from(props: &UploadInfoDialogProps) -> Self {
        let mut open = props.open;
        let on_cancel = EventHandler::new(move |_event: MouseEvent| open.set(false));
        let on_primary = EventHandler::new(move |_event: MouseEvent| {
            open.set(false);
            upload::trigger();
        });
        let title = TITLE;
        let intro = INTRO;
        let warning = None;
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
