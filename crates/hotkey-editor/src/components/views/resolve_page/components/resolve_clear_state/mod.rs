pub mod components;
mod style;
use components::resolve_clear_icon::ResolveClearIcon;
use components::resolve_clear_label::ResolveClearLabel;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveClearState);

/// The "nothing to resolve" state shown when a file is loaded but conflict-free.
#[component]
pub fn ResolveClearState() -> Element {
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "clear",
            ResolveClearIcon {}
            ResolveClearLabel {}
        }
    }
}
