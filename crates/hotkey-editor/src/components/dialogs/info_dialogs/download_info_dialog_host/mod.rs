use crate::components::dialogs::info_dialogs::download_info_dialog::{
    DownloadInfoDialog, DownloadInfoDialogProps,
};
use crate::services::customkeys::service::CustomKeysService;
use crate::services::files::download::BlobDownload;
use dioxus::prelude::*;

/// Connected wrapper for the download dialog: reads the live document from the
/// [`CustomKeysService`] and owns the confirm handler that serializes and
/// downloads it. Neither the export button nor the burger menu touches the
/// document to offer a download — they place this host and pass only the open
/// signal. Owns no markup beyond the dialog it wraps.
#[component]
pub fn DownloadInfoDialogHost(open: Signal<bool>) -> Element {
    let custom_keys_service = use_context::<CustomKeysService>();
    let keys = custom_keys_service.keys();
    let on_confirm = EventHandler::new(move |_event: ()| {
        let serialized = {
            let read_guard = keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.normalize().to_string()
        };
        BlobDownload::trigger("CustomKeys.txt", &serialized);
    });
    let dialog = DownloadInfoDialogProps { open, on_confirm };
    rsx! {
        DownloadInfoDialog { ..dialog }
    }
}
