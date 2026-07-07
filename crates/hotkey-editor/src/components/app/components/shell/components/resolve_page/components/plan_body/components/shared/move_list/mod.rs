mod props;
mod style;
use dioxus::prelude::*;
pub use props::MoveListProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveList);
#[component]
pub fn MoveList(props: MoveListProps) -> Element {
    let data_category = props.data_category;
    let children = props.children;
    rsx! { div { class: CLASS, "data-category": data_category, {children} } }
}
