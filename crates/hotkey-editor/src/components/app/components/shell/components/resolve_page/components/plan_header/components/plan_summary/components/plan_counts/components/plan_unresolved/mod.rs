mod props;
mod style;
use dioxus::prelude::*;
pub use props::PlanUnresolvedProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PlanUnresolved);
#[component]
pub fn PlanUnresolved(props: PlanUnresolvedProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count} unresolved" } }
}
