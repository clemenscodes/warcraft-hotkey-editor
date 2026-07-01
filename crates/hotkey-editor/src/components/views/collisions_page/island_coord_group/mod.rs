mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::IslandCoordGroupProps;
use style::CLASS;
assert_component!(IslandCoordGroup);
/// The column/row coordinate pair.
#[component]
pub fn IslandCoordGroup(props: IslandCoordGroupProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
