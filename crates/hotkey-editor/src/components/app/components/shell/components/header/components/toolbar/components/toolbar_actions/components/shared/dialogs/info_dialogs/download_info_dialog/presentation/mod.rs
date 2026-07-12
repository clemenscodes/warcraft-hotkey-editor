use super::data::{INTRO, PRIMARY_LABEL, TITLE, WARNING};
use super::model::DownloadInfoDialogModel;
use dioxus::prelude::*;

/// The download dialog's shaped data: the open value it drives, the change handler
/// mirroring the headless dialog's own close, the shared copy and warning, and the
/// cancel and confirm handlers. The shared `InfoDialog` is placed with these as
/// named fields.
pub(super) struct DownloadInfoDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: &'static str,
    pub(super) intro: &'static str,
    pub(super) warning: Option<&'static str>,
    pub(super) primary_label: &'static str,
    pub(super) on_primary: EventHandler<MouseEvent>,
    pub(super) on_cancel: EventHandler<MouseEvent>,
}

impl From<&DownloadInfoDialogModel> for DownloadInfoDialogPresentation {
    fn from(props: &DownloadInfoDialogModel) -> Self {
        let open = props.open;
        let on_open_change = props.on_open_change;
        let on_confirm = props.on_confirm;
        let cancel_change = on_open_change;
        let on_cancel = EventHandler::new(move |_event: MouseEvent| cancel_change.call(false));
        let primary_change = on_open_change;
        let on_primary = EventHandler::new(move |_event: MouseEvent| {
            primary_change.call(false);
            on_confirm.call(());
        });
        let title = TITLE;
        let intro = INTRO;
        let warning = Some(WARNING);
        let primary_label = PRIMARY_LABEL;
        Self {
            open,
            on_open_change,
            title,
            intro,
            warning,
            primary_label,
            on_primary,
            on_cancel,
        }
    }
}

impl ddd::Presentation for DownloadInfoDialogPresentation {
    type Model = DownloadInfoDialogModel;
}
