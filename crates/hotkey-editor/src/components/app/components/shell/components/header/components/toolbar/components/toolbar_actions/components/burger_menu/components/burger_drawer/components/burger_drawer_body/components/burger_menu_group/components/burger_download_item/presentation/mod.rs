use super::data::LABEL;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_DOWNLOAD;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::prelude::*;

/// The burger download row's shaped data: whether it is hidden (no file loaded yet), the fixed
/// icon and label, its idle weight, whether the download-info dialog is open, the click handler
/// that opens it, and the change handler the mounted dialog mirrors its own close through. The
/// open signal is local and owned here — the row is the button that opens the dialog, so it owns
/// the signal and the dialog travels with it.
pub(super) struct BurgerDownloadItemPresentation {
    pub(super) hidden: bool,
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) role: Option<&'static str>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Reads the live document from the CustomKeys service to hide the row until a file is loaded,
/// owns the download-info dialog's local open signal, and shapes the row: the click handler that
/// opens the dialog and the change handler the mounted dialog mirrors its own close through.
pub(super) fn use_burger_download_item() -> BurgerDownloadItemPresentation {
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let has_file_memo = use_memo(move || keys.read().is_some());
    let has_file = has_file_memo();
    let hidden = !has_file;
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| open_signal.set(true));
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    BurgerDownloadItemPresentation {
        hidden,
        icon: ICON_DOWNLOAD,
        label: String::from(LABEL),
        state: BurgerItemState::Idle,
        role: Some("menuitem"),
        open,
        onclick,
        on_open_change,
    }
}
