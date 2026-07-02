mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CarriersGridProps;
use style::CLASS;
assert_component!(CarriersGrid);
#[component]
pub fn CarriersGrid(props: CarriersGridProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
