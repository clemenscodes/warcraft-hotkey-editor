use super::components::export_button::ExportButtonProps;
use crate::services::customkeys::service::CustomKeysService;
use dioxus::prelude::*;

/// The seam: reads the live document from the [`CustomKeysService`] to decide whether
/// there is anything to export, and shapes the export button's props. The download
/// itself is owned by `DownloadInfoDialogHost`, so this only decides visibility and
/// opens the dialog.
pub(super) fn use_export_button() -> ExportButtonProps {
    let custom_keys_service = use_context::<CustomKeysService>();
    let keys = custom_keys_service.keys();
    let visible = keys.read().is_some();
    let mut info_open = use_signal(|| false);
    let on_open = EventHandler::new(move |_event: MouseEvent| info_open.set(true));
    ExportButtonProps {
        visible,
        info_open,
        on_open,
    }
}
