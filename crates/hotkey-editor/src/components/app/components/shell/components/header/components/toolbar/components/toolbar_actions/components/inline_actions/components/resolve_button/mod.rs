mod hooks;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_resolve_button;

/// Toolbar button that navigates to the Resolve page, where the cascade plan is
/// previewed and applied. Reads the live document and router from context: disabled
/// until a file is loaded, clicking routes to the Resolve page.
#[component]
pub fn ResolveButton() -> Element {
    let button = use_resolve_button();
    rsx! {
        ToolbarButton { ..button }
    }
}
