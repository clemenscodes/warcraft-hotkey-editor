pub mod components;
mod props;
mod style;
use components::resolve_plan_counts::{ResolvePlanCounts, ResolvePlanCountsProps};
use components::resolve_plan_title::ResolvePlanTitle;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolvePlanSummaryProps;
use style::CLASS;
assert_component!(ResolvePlanSummary);

/// The plan title over its move/unresolved counts.
#[component]
pub fn ResolvePlanSummary(props: ResolvePlanSummaryProps) -> Element {
    let counts = ResolvePlanCountsProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            ResolvePlanTitle {}
            ResolvePlanCounts { ..counts }
        }
    }
}
