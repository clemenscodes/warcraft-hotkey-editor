use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialog;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::files::download;
use dioxus::prelude::*;
use tw_macro::assert_component;

/// Connected wrapper for the download dialog: reads the live document from the
/// [`CustomKeysService`](crate::services::customkeys::service::CustomKeysService) and owns the confirm handler that serializes and
/// downloads it. Neither the export button nor the burger menu touches the
/// document to offer a download — they place this host and pass only the open
/// signal. Owns no markup beyond the dialog it wraps.
#[component]
pub fn DownloadInfoDialogHost(open: Signal<bool>) -> Element {
    let custom_keys_service = use_custom_keys_service();
    let on_confirm = EventHandler::new(move |_event: ()| {
        // R5: the download IS the stored CustomKeys.txt text — read it back, never
        // re-serialize or re-normalize the in-memory aggregate.
        let serialized = custom_keys_service.exported_text();
        download::trigger("CustomKeys.txt", &serialized);
    });
    rsx! {
        DownloadInfoDialog { open, on_confirm }
    }
}

assert_component!(DownloadInfoDialogHost);
