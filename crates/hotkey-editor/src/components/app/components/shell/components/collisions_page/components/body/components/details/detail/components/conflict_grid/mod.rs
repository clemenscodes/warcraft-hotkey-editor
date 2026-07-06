mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictGrid);
#[component]
pub fn ConflictGrid(props: ConflictGridProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
