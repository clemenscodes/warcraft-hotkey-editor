mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::MoveListProps;
use style::CLASS;
assert_component!(MoveList);
#[component]
pub fn MoveList(props: MoveListProps) -> Element {
    let data_category = props.data_category;
    let children = props.children;
    rsx! { div { class: CLASS, "data-category": data_category, {children} } }
}
