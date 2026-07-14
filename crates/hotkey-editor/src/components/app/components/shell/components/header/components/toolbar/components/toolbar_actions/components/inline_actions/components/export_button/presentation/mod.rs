use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_DOWNLOAD;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

/// The download button's shaped data: whether it is hidden (no file loaded yet), the fixed icon
/// and label, whether the download-info dialog is open, the click handler that opens it, and the
/// change handler the mounted dialog mirrors its own close through. The open signal is local and
/// owned here — the button that opens the dialog owns it, so the dialog travels with it.
pub(super) struct ExportButtonPresentation {
    pub(super) hidden: bool,
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the live document from the CustomKeys service to hide the button until a file is
/// loaded, owns the download-info dialog's local open signal, and shapes the button's data: the
/// click handler that opens the dialog and the change handler the mounted dialog mirrors its own
/// close through.
pub(super) fn use_export_button() -> ExportButtonPresentation {
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let has_file_memo = use_memo(move || keys.read().is_some());
    let has_file = has_file_memo();
    let hidden = !has_file;
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| open_signal.set(true));
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    ExportButtonPresentation {
        hidden,
        icon: ICON_DOWNLOAD,
        aria_label: ARIA_LABEL,
        open,
        onclick,
        on_open_change,
    }
}
