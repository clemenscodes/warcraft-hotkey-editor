mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ActiveAccent);

/// The active tab's accent ring: an overlay the active variant stacks on top of the base
/// button to mark it current. It draws only the accent border and glow and captures no
/// pointer events, so the base button beneath stays the interactive element.
#[component]
pub fn ActiveAccent() -> Element {
    rsx! {
        div { class: CLASS }
    }
}
