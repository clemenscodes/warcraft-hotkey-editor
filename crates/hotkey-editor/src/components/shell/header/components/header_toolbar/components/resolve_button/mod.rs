mod hooks;
mod props;

use crate::components::shell::header::components::header_toolbar::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use hooks::use_resolve_button;
pub use props::ResolveButtonProps;

/// Toolbar button that navigates to the Resolve page, where the cascade plan is
/// previewed and applied. (Replaces the old confirm dialog.)
#[component]
pub fn ResolveButton(props: ResolveButtonProps) -> Element {
    let button = use_resolve_button(&props);
    rsx! {
        ToolbarButton { ..button }
    }
}
