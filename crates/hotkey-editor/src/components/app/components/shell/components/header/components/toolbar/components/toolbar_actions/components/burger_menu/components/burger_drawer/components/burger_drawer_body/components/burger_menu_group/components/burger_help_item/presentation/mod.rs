use super::data::LABEL;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_HELP;
use dioxus::prelude::*;

/// The burger help row's shaped data: the fixed icon and label, its idle weight, its aria
/// state, whether its guide is open, the toggle handler, and the change handler the mounted
/// dialog mirrors its own close through. The open signal is local and owned here — the row is
/// the button that opens the dialog, so it owns the signal and the dialog travels with it.
pub(super) struct BurgerHelpItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) role: Option<&'static str>,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<&'static str>,
    pub(super) open: bool,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) on_open_change: Callback<bool>,
}

/// Owns the burger's own help-guide open signal (closed until the row is tapped — the inline
/// help button owns the first-visit onboarding) and shapes the row: the toggle handler that
/// opens or closes the guide, and the change handler the mounted dialog mirrors its own close
/// through. Tapping the row does not close the drawer, so the row stays mounted to keep the
/// dialog it owns alive.
pub(super) fn use_burger_help_item() -> BurgerHelpItemPresentation {
    let mut open_signal = use_signal::<bool>(|| false);
    let open = open_signal();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*open_signal.read();
        open_signal.set(next);
    });
    let on_open_change = Callback::new(move |is_open: bool| open_signal.set(is_open));
    let aria_expanded = if open { Some("true") } else { Some("false") };
    BurgerHelpItemPresentation {
        icon: ICON_HELP,
        label: String::from(LABEL),
        state: BurgerItemState::Idle,
        role: Some("menuitem"),
        aria_haspopup: Some("dialog"),
        aria_expanded,
        open,
        onclick,
        on_open_change,
    }
}
