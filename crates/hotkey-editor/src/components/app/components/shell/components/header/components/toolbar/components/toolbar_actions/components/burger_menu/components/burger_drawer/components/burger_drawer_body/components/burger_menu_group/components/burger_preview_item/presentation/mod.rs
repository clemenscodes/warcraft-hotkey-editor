use super::data::HIDE_LABEL;
use super::data::LABEL;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_PREVIEW;
use dioxus::prelude::*;

/// The burger preview row's shaped data: the fixed icon, its flipping label, its idle weight,
/// its aria state, whether its preview is open, the toggle handler, and the change handler the
/// mounted dialog mirrors its own close through. The open signal is local and owned here — the
/// row is the button that opens the dialog, so it owns the signal and the dialog travels with
/// it. The preview toggles rather than opening a one-shot dialog, so it reports its state with
/// `aria-pressed` and carries no `aria-haspopup`/`aria-expanded`.
pub(super) struct BurgerPreviewItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) role: Option<&'static str>,
    pub(super) aria_pressed: Option<&'static str>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Owns the burger's own preview open signal (closed until the row is tapped) and shapes the
/// row: the toggle handler that opens or closes the preview, the label that flips to the hide
/// wording while it is open, its pressed state, and the change handler the mounted dialog
/// mirrors its own close through. Tapping the row does not close the drawer, so the row stays
/// mounted to keep the dialog it owns alive.
pub(super) fn use_burger_preview_item() -> BurgerPreviewItemPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let label = if open { HIDE_LABEL } else { LABEL };
    let aria_pressed = if open { Some("true") } else { Some("false") };
    BurgerPreviewItemPresentation {
        icon: ICON_PREVIEW,
        label: String::from(label),
        state: BurgerItemState::Idle,
        role: Some("menuitem"),
        aria_pressed,
        open,
        onclick,
        on_open_change,
    }
}
