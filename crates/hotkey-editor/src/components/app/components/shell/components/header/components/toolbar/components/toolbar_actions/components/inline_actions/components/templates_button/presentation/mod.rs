use crate::components::app::components::shell::components::shared::icons::ICON_TEMPLATES;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The templates button's shaped view: its icon, label, popup/expanded aria state,
/// and click handler.
pub(super) struct TemplatesButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the overlay context and shapes the templates button: clicking toggles the
/// browser dialog, and `aria-expanded` tracks whether it is open.
pub(super) fn use_templates_button() -> TemplatesButtonModel {
    let overlay = use_overlay_state();
    let mut templates_dialog_open = overlay.templates_dialog_open();
    let is_open = templates_dialog_open();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
    });
    TemplatesButtonModel {
        icon: ICON_TEMPLATES,
        aria_label: "Browse layout templates",
        aria_haspopup: Some("dialog"),
        aria_expanded: Some(is_open),
        onclick,
    }
}
