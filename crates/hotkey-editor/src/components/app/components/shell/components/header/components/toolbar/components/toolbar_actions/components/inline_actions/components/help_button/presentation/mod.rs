use crate::components::app::components::shell::components::shared::icons::ICON_HELP;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The help button's shaped view: its icon, label, popup/expanded aria state, and
/// click handler.
pub(super) struct HelpButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the overlay context and shapes the help button: clicking opens the
/// onboarding dialog, and `aria-expanded` tracks whether it is open.
pub(super) fn use_help_button() -> HelpButtonModel {
    let overlay = use_overlay_state();
    let mut help_open = overlay.help_open();
    let is_open = help_open();
    let onclick = EventHandler::new(move |_event: MouseEvent| help_open.set(true));
    HelpButtonModel {
        icon: ICON_HELP,
        aria_label: "How to use this editor",
        aria_haspopup: Some("dialog"),
        aria_expanded: Some(is_open),
        onclick,
    }
}
