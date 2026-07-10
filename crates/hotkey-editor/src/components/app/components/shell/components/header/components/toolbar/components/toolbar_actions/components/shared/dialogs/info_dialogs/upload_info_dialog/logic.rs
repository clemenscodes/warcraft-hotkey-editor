use super::data::{INTRO, PRIMARY_LABEL, TITLE};
use super::props::UploadInfoDialogProps;
use crate::services::files::upload;
use dioxus::prelude::*;

/// The import dialog's shaped data: the open signal it drives, the shared copy, and
/// the cancel and choose-file handlers. The shared `InfoDialog` is placed with these
/// as named fields.
pub(super) struct UploadInfoDialogModel {
    pub(super) open: Signal<bool>,
    pub(super) title: &'static str,
    pub(super) intro: &'static str,
    pub(super) warning: Option<&'static str>,
    pub(super) primary_label: &'static str,
    pub(super) on_primary: EventHandler<MouseEvent>,
    pub(super) on_cancel: EventHandler<MouseEvent>,
}

impl From<&UploadInfoDialogProps> for UploadInfoDialogModel {
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
