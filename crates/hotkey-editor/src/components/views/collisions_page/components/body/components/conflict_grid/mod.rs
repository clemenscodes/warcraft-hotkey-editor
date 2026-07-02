mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictGridProps;
use style::CLASS;
assert_component!(ConflictGrid);
#[component]
pub fn ConflictGrid(props: ConflictGridProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
