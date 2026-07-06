mod props;
mod style;
use dioxus::prelude::*;
pub use props::MoveCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveCard);
#[component]
pub fn MoveCard(props: MoveCardProps) -> Element {
    let is_stuck = props.is_stuck;
    let children = props.children;
    rsx! { div { class: CLASS, "data-stuck": is_stuck, {children} } }
}
