mod props;
mod style;
use dioxus::prelude::*;
pub use props::GridColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(GridColumn);
#[component]
pub fn GridColumn(props: GridColumnProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
