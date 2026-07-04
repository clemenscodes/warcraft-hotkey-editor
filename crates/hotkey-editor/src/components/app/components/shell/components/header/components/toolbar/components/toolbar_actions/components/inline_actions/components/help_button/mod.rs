mod hooks;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_help_button;

/// Toolbar button that opens the onboarding help dialog.
#[component]
pub fn HelpButton() -> Element {
    let button = use_help_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
