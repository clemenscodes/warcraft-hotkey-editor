use super::data::LABEL;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_UPLOAD;
use dioxus::prelude::*;

/// The burger upload row's shaped data: the fixed icon and label, its idle weight, whether its
/// info dialog is open, the open handler, and the change handler the mounted dialog mirrors its
/// own close through. The open signal is local and owned here — the row is the button that
/// opens the dialog, so it owns the signal and the dialog travels with it. The upload flow is a
/// one-shot info dialog (opened on click, never toggled), so the row carries no aria
/// popup/expanded/pressed state.
pub(super) struct BurgerUploadItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) role: Option<&'static str>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Owns the burger's own upload info-dialog open signal (closed until the row is tapped) and
/// shapes the row: the open handler that raises the info dialog, and the change handler the
/// mounted dialog mirrors its own close through. The dialog's confirm triggers the hidden file
/// input owned by the inline upload button, so this row opens the dialog and never picks a
/// file itself. Tapping the row does not close the drawer, so the row stays mounted to keep the
/// dialog it owns alive.
pub(super) fn use_burger_upload_item() -> BurgerUploadItemPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| open_signal.set(true));
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    BurgerUploadItemPresentation {
        icon: ICON_UPLOAD,
        label: String::from(LABEL),
        state: BurgerItemState::Idle,
        role: Some("menuitem"),
        open,
        onclick,
        on_open_change,
    }
}
