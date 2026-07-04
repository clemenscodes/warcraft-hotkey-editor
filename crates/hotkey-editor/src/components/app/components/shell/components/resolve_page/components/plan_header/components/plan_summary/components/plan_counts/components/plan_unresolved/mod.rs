mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::PlanUnresolvedProps;
use style::CLASS;
assert_component!(PlanUnresolved);
#[component]
pub fn PlanUnresolved(props: PlanUnresolvedProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count} unresolved" } }
}
