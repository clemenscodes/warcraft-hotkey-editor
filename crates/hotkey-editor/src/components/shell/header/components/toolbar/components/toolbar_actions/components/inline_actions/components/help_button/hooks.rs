use crate::components::shell::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::shared::icons::ICON_HELP;
use crate::services::overlay_state::OverlayState;
use dioxus::prelude::*;

/// Reads the overlay context and shapes the help button: clicking opens the
/// onboarding dialog, and `aria-expanded` tracks whether it is open.
pub(super) fn use_help_button() -> ToolbarButtonProps {
    let overlay = use_context::<OverlayState>();
    let mut help_open = overlay.help_open;
    let is_open = help_open();
    let onclick = EventHandler::new(move |_event: MouseEvent| help_open.set(true));
    ToolbarButtonProps {
        icon: ICON_HELP,
        aria_label: "How to use this editor",
        aria_haspopup: Some("dialog"),
        aria_expanded: Some(is_open),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
