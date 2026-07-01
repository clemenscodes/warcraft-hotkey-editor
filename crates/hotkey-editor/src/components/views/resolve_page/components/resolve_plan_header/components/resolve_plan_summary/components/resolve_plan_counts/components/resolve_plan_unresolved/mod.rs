mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolvePlanUnresolvedProps;
use style::CLASS;
assert_component!(ResolvePlanUnresolved);
#[component]
pub fn ResolvePlanUnresolved(props: ResolvePlanUnresolvedProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count} unresolved" } }
}
