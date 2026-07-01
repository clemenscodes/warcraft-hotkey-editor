mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveMoveListProps;
use style::CLASS;
assert_component!(ResolveMoveList);
#[component]
pub fn ResolveMoveList(props: ResolveMoveListProps) -> Element {
    let data_category = props.data_category;
    let children = props.children;
    rsx! { div { class: CLASS, "data-category": data_category, {children} } }
}
