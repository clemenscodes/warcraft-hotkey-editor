use super::data::ARIA_LABEL;
use crate::components::app::components::shell::components::shared::icons::ICON_DOWNLOAD;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

pub(super) struct ExportButtonPresentation {
    pub(super) hidden: bool,
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

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
