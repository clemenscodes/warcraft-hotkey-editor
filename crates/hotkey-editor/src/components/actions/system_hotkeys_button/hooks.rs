use crate::components::shared::icons::ICON_COG;
use crate::components::shared::toolbar_button::ToolbarButtonProps;
use crate::services::overlay_state::OverlayState;
use dioxus::prelude::*;

/// Reads the overlay context and shapes the general-hotkeys button: clicking
/// toggles the dialog, and `aria-expanded` tracks whether it is open.
pub(super) fn use_system_hotkeys_button() -> ToolbarButtonProps {
    let overlay = use_context::<OverlayState>();
    let mut system_hotkeys_open = overlay.system_hotkeys_open;
    let is_open = system_hotkeys_open();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
    });
    ToolbarButtonProps {
        icon: ICON_COG,
        aria_label: "General hotkeys",
        aria_haspopup: Some("dialog"),
        aria_expanded: Some(is_open),
        onclick,
        ..ToolbarButtonProps::default()
    }
}
