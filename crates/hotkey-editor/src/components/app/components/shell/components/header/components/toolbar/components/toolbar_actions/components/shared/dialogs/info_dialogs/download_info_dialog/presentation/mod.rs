use super::data::{INTRO, PRIMARY_LABEL, TITLE, WARNING};
use super::model::DownloadInfoDialogModel;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::files::download;
use dioxus::prelude::*;

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

pub(super) fn use_download_info_dialog(
    props: &DownloadInfoDialogModel,
) -> DownloadInfoDialogPresentation {
    let custom_keys_service = use_custom_keys_service();
    let open = props.open;
    let on_open_change = props.on_open_change;
    let cancel_change = on_open_change;
    let on_cancel = EventHandler::new(move |_event: MouseEvent| cancel_change.call(false));
    let primary_change = on_open_change;
    let on_primary = EventHandler::new(move |_event: MouseEvent| {
        primary_change.call(false);
        let serialized = custom_keys_service.exported_text();
        download::trigger("CustomKeys.txt", &serialized);
    });
    let title = TITLE;
    let intro = INTRO;
    let warning = Some(WARNING);
    let primary_label = PRIMARY_LABEL;
    DownloadInfoDialogPresentation {
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

impl ddd::Presentation for DownloadInfoDialogPresentation {
    type Model = DownloadInfoDialogModel;
}
