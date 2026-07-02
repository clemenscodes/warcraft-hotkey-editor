mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::MoveCardProps;
use style::CLASS;
assert_component!(MoveCard);
#[component]
pub fn MoveCard(props: MoveCardProps) -> Element {
    let is_stuck = props.is_stuck;
    let children = props.children;
    rsx! { div { class: CLASS, "data-stuck": is_stuck, {children} } }
}
