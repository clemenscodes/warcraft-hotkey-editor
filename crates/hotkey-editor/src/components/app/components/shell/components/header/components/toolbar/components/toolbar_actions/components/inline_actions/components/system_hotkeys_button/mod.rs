mod hooks;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_system_hotkeys_button;

/// Toolbar button that opens the general (system) hotkeys dialog.
#[component]
pub fn SystemHotkeysButton() -> Element {
    let button = use_system_hotkeys_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
