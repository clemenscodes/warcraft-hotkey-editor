mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_resolve_button;
use style::CLASS;
use tw_macro::assert_component;

/// Toolbar button that navigates to the Resolve page, where the cascade plan is
/// previewed and applied. Reads the live document and router from context: disabled
/// until a file is loaded, clicking routes to the Resolve page. Its slot is hidden below
/// laptop, where the burger drawer offers the action instead.
#[component]
pub fn ResolveButton() -> Element {
    let button = use_resolve_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton { ..button }
        }
    }
}

assert_component!(ResolveButton);
