mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::GridColumnProps;
use style::CLASS;
assert_component!(GridColumn);
#[component]
pub fn GridColumn(props: GridColumnProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
