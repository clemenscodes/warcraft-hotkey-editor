mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveMoveArrow);

/// The centred → between the from and to grids of a move.
#[component]
pub fn ResolveMoveArrow() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "\u{2192}" } }
}
