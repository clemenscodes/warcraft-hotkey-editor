pub mod components;
mod hooks;

use components::resolve_button::ResolveButton;
use dioxus::prelude::*;
use hooks::use_resolve_button;

/// Connected wrapper: feeds `ResolveButton` the live disabled-state and click
/// handler from context, and owns no markup beyond the single leaf it wraps.
#[component]
pub fn ResolveButtonHost() -> Element {
    let button = use_resolve_button();
    rsx! {
        ResolveButton { ..button }
    }
}
