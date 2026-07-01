mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictCardProps;
use style::CLASS;
assert_component!(ConflictCard);
#[component]
pub fn ConflictCard(props: ConflictCardProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
