use dioxus::prelude::*;

use crate::components::shared::icons::ICON_COG;
use crate::components::shared::toolbar_button::ToolbarButton;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysButtonProps {
    pub system_hotkeys_open: Signal<bool>,
}

/// Toolbar button that opens the general (system) hotkeys dialog.
#[component]
pub fn SystemHotkeysButton(props: SystemHotkeysButtonProps) -> Element {
    let mut system_hotkeys_open = props.system_hotkeys_open;
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
