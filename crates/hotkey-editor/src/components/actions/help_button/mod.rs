use dioxus::prelude::*;

use crate::components::shared::icons::ICON_HELP;
use crate::components::shared::toolbar_button::ToolbarButton;

#[derive(Props, Clone, PartialEq)]
pub struct HelpButtonProps {
    pub help_open: Signal<bool>,
}

/// Toolbar button that opens the onboarding help dialog.
#[component]
pub fn HelpButton(props: HelpButtonProps) -> Element {
    let mut help_open = props.help_open;
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
