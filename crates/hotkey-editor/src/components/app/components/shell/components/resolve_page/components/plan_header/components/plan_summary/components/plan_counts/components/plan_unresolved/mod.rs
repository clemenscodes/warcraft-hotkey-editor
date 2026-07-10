mod props;
mod style;
use dioxus::prelude::*;
use props::PlanUnresolvedProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn PlanUnresolved(props: PlanUnresolvedProps) -> Element {
    let count = props.count;
    rsx! { span { class: CLASS, "{count} unresolved" } }
}

assert_component!(PlanUnresolved);
