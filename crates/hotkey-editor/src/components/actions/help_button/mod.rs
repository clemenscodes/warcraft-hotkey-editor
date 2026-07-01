use dioxus::prelude::*;

use crate::components::shared::icons::ICON_HELP;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::overlay_state::OverlayState;

/// Toolbar button that opens the onboarding help dialog.
#[component]
pub fn HelpButton() -> Element {
    let overlay = use_context::<OverlayState>();
    let mut help_open = overlay.help_open;
    let is_open = help_open();
    let open_help = move |_| help_open.set(true);
    rsx! {
        ToolbarButton {
            icon: ICON_HELP,
            aria_label: "How to use this editor",
            aria_haspopup: "dialog",
            aria_expanded: is_open,
            onclick: open_help,
        }
    }
}
