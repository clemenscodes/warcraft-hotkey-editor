mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::IslandDetailHeaderProps;
use style::CLASS;
assert_component!(IslandDetailHeader);
#[component]
pub fn IslandDetailHeader(props: IslandDetailHeaderProps) -> Element {
    let children = props.children;
    rsx! { header { class: CLASS, {children} } }
}
