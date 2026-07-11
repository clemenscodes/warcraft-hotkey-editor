use crate::components::app::components::shell::components::shared::icons::ICON_COG;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;

/// The general-hotkeys button's shaped view: its icon, label, popup/expanded aria
/// state, and click handler.
pub(super) struct SystemHotkeysButtonModel {
    pub(super) icon: &'static str,
    pub(super) aria_label: &'static str,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_expanded: Option<bool>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

/// Reads the overlay context and shapes the general-hotkeys button: clicking
/// toggles the dialog, and `aria-expanded` tracks whether it is open.
pub(super) fn use_system_hotkeys_button() -> SystemHotkeysButtonModel {
    let overlay = use_overlay_state();
    let mut system_hotkeys_open = overlay.system_hotkeys_open();
    let is_open = system_hotkeys_open();
    let onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
    });
    SystemHotkeysButtonModel {
        icon: ICON_COG,
        aria_label: "General hotkeys",
        aria_haspopup: Some("dialog"),
        aria_expanded: Some(is_open),
        onclick,
    }
}
