mod hooks;

use crate::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_templates_button;

/// Toolbar button that opens the layout templates browser.
#[component]
pub fn TemplatesButton() -> Element {
    let button = use_templates_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
