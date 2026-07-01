mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveGridColProps;
use style::CLASS;
assert_component!(ResolveGridCol);
#[component]
pub fn ResolveGridCol(props: ResolveGridColProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
