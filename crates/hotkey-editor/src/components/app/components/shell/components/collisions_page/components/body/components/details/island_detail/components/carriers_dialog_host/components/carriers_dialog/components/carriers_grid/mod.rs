mod props;
mod style;
use dioxus::prelude::*;
pub use props::CarriersGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CarriersGrid);
#[component]
pub fn CarriersGrid(props: CarriersGridProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
