use super::props::ExportButtonProps;
use crate::components::shell::header::components::header_toolbar::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::dialogs::info_dialogs::download_info_dialog::DownloadInfoDialogProps;
use crate::components::shared::icons::ICON_DOWNLOAD;
use crate::services::files::download::BlobDownload;
use dioxus::prelude::*;

/// The export button's shaped view: whether a file is loaded (so the button
/// renders at all), the info dialog signal, and the open/confirm handlers.
pub(super) struct ExportButtonModel {
    pub(super) visible: bool,
    pub(super) info_open: Signal<bool>,
    pub(super) on_open: EventHandler<MouseEvent>,
    pub(super) on_confirm: EventHandler<()>,
}

/// Serializing the normalized file and triggering the blob download live here so
/// the body stays pure RSX.
pub(super) fn use_export_button(props: &ExportButtonProps) -> ExportButtonModel {
    let loaded_keys = props.loaded_keys;
    let visible = loaded_keys.read().is_some();
    let mut info_open = use_signal(|| false);
    let on_open = EventHandler::new(move |_event: MouseEvent| info_open.set(true));
    let on_confirm = EventHandler::new(move |_event: ()| {
        let serialized = {
            let read_guard = loaded_keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.normalize().to_string()
        };
        BlobDownload::trigger("CustomKeys.txt", &serialized);
    });
    ExportButtonModel {
        visible,
        info_open,
        on_open,
        on_confirm,
    }
}

impl From<&ExportButtonModel> for ToolbarButtonProps {
    fn from(model: &ExportButtonModel) -> Self {
        let onclick = model.on_open;
        Self {
            icon: ICON_DOWNLOAD,
            aria_label: "Download CustomKeys.txt",
            onclick,
            ..Self::default()
        }
    }
}

impl From<&ExportButtonModel> for DownloadInfoDialogProps {
    fn from(model: &ExportButtonModel) -> Self {
        let open = model.info_open;
        let on_confirm = model.on_confirm;
        Self { open, on_confirm }
    }
}
