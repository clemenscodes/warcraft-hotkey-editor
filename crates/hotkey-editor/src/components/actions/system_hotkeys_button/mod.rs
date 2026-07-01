use dioxus::prelude::*;

use crate::components::shared::icons::ICON_COG;
use crate::components::shared::toolbar_button::ToolbarButton;
use crate::services::overlay_state::OverlayState;

/// Toolbar button that opens the general (system) hotkeys dialog.
#[component]
pub fn SystemHotkeysButton() -> Element {
    let overlay = use_context::<OverlayState>();
    let mut system_hotkeys_open = overlay.system_hotkeys_open;
    let is_open = system_hotkeys_open();
    let toggle_system_hotkeys = move |_| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
    };
    rsx! {
        ToolbarButton {
            icon: ICON_COG,
            aria_label: "General hotkeys",
            aria_haspopup: "dialog",
            aria_expanded: is_open,
            onclick: toggle_system_hotkeys,
        }
    }
}
