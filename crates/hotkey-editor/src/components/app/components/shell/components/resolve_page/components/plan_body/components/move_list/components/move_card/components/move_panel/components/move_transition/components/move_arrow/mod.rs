mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The centred → between the from and to grids of a move.
#[component]
pub fn MoveArrow() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "\u{2192}" } }
}

assert_component!(MoveArrow);
