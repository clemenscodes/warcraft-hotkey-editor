use crate::components::shared::icons::ICON_TEMPLATES;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::overlay_state::OverlayState;
use dioxus::prelude::*;

/// Toolbar button that opens the layout templates browser.
#[component]
pub fn TemplatesButton() -> Element {
    let overlay = use_context::<OverlayState>();
    let mut templates_dialog_open = overlay.templates_dialog_open;
    let is_open = templates_dialog_open();
    let toggle_templates = move |_| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
    };
    rsx! {
        ToolbarButton {
            icon: ICON_TEMPLATES,
            aria_label: "Browse layout templates",
            aria_haspopup: "dialog",
            aria_expanded: is_open,
            onclick: toggle_templates,
        }
    }
}
