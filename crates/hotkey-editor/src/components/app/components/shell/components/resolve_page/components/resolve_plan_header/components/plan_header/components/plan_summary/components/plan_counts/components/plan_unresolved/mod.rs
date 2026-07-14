mod model;
mod view;

pub use view::PlanUnresolvedView;
mod style;
use dioxus::prelude::*;
use model::PlanUnresolvedModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn PlanUnresolved(props: PlanUnresolvedModel) -> Element {
    let count = props.count;
    rsx! {
        span {
            class: CLASS,
            "{count} unresolved"
        }
    }
}

assert_component!(PlanUnresolved);
