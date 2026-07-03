use super::components::export_button::ExportButtonProps;
use crate::services::customkeys::service::CustomKeysService;
use crate::services::files::download::BlobDownload;
use dioxus::prelude::*;

/// The seam: reads the live document from the [`CustomKeysService`], decides whether
/// there is anything to export, and shapes the export button's props. Serializing the
/// normalized file and triggering the blob download live here so the leaf stays pure.
pub(super) fn use_export_button() -> ExportButtonProps {
    let custom_keys_service = use_context::<CustomKeysService>();
    let keys = custom_keys_service.keys();
    let visible = keys.read().is_some();
    let mut info_open = use_signal(|| false);
    let on_open = EventHandler::new(move |_event: MouseEvent| info_open.set(true));
    let on_confirm = EventHandler::new(move |_event: ()| {
        let serialized = {
            let read_guard = keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.to_string()
        };
        BlobDownload::trigger("CustomKeys.txt", &serialized);
    });
    ExportButtonProps {
        visible,
        info_open,
        on_open,
        on_confirm,
    }
}
