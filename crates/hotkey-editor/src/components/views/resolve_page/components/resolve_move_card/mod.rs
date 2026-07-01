mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveMoveCardProps;
use style::CLASS;
assert_component!(ResolveMoveCard);
#[component]
pub fn ResolveMoveCard(props: ResolveMoveCardProps) -> Element {
    let is_stuck = props.is_stuck;
    let children = props.children;
    rsx! { div { class: CLASS, "data-stuck": is_stuck, {children} } }
}
