use crate::components::shell::header::components::header_toolbar::components::shared::toolbar_button::ToolbarButtonProps;
use crate::components::shared::icons::ICON_TEMPLATES;
use crate::services::overlay_state::OverlayState;
use dioxus::prelude::*;

/// Reads the overlay context and shapes the templates button: clicking toggles the
/// browser dialog, and `aria-expanded` tracks whether it is open.
pub(super) fn use_templates_button() -> ToolbarButtonProps {
    let overlay = use_context::<OverlayState>();
    let mut templates_dialog_open = overlay.templates_dialog_open;
    let is_open = templates_dialog_open();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
    });
    ToolbarButtonProps {
        icon: ICON_TEMPLATES,
        aria_label: "Browse layout templates",
        aria_haspopup: Some("dialog"),
        aria_expanded: Some(is_open),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
