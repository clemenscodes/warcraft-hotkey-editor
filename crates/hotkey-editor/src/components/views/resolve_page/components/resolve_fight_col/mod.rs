mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveFightColProps;
use style::CLASS;
assert_component!(ResolveFightCol);
#[component]
pub fn ResolveFightCol(props: ResolveFightColProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
