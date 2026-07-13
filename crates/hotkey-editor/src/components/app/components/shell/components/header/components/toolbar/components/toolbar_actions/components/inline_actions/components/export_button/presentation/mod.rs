use crate::components::app::components::shell::components::shared::icons::ICON_DOWNLOAD;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

/// What the export button's body renders: whether there is a file to export at all,
/// the download-info dialog's open signal, and the shaped toolbar button's fields.
pub(super) struct ExportButtonPresentation {
    pub visible: bool,
    pub info_open: Signal<bool>,
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

/// The seam: reads the live document from the [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService) to decide whether
/// there is anything to export, owns the info dialog's open signal, and shapes the
/// toolbar button. Clicking opens the dialog; the download itself is owned by
/// `DownloadInfoDialog`.
pub(super) fn use_export_button() -> ExportButtonPresentation {
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let visible_memo = use_memo(move || keys.read().is_some());
    let visible = visible_memo();
    let mut info_open = use_signal(|| false);
    let on_open = EventHandler::new(move |_event: MouseEvent| info_open.set(true));
    ExportButtonPresentation {
        visible,
        info_open,
        icon: ICON_DOWNLOAD,
        aria_label: "Download CustomKeys.txt",
        onclick: on_open,
    }
}
