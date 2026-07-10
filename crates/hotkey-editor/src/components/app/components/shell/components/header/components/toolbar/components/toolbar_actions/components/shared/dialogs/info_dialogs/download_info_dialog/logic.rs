use super::data::{INTRO, PRIMARY_LABEL, TITLE, WARNING};
use super::props::DownloadInfoDialogProps;
use dioxus::prelude::*;

/// The download dialog's shaped data: the open signal it drives, the shared copy
/// and warning, and the cancel and confirm handlers. The shared `InfoDialog` is
/// placed with these as named fields.
pub(super) struct DownloadInfoDialogModel {
    pub(super) open: Signal<bool>,
    pub(super) title: &'static str,
    pub(super) intro: &'static str,
    pub(super) warning: Option<&'static str>,
    pub(super) primary_label: &'static str,
    pub(super) on_primary: EventHandler<MouseEvent>,
    pub(super) on_cancel: EventHandler<MouseEvent>,
}

impl From<&DownloadInfoDialogProps> for DownloadInfoDialogModel {
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
