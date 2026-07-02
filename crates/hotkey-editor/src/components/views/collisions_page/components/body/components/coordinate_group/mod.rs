mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CoordinateGroupProps;
use style::CLASS;
assert_component!(CoordinateGroup);
/// The column/row coordinate pair.
#[component]
pub fn CoordinateGroup(props: CoordinateGroupProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
