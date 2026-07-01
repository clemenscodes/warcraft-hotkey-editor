mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveFightRowProps;
use style::CLASS;
assert_component!(ResolveFightRow);
#[component]
pub fn ResolveFightRow(props: ResolveFightRowProps) -> Element {
    let children = props.children;
    rsx! { div { class: CLASS, {children} } }
}
