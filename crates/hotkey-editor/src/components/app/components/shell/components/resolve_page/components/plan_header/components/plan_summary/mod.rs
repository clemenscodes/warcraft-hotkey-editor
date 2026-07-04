pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::plan_counts::{PlanCounts, PlanCountsProps};
use components::plan_title::PlanTitle;
use dioxus::prelude::*;
pub use props::PlanSummaryProps;
use style::CLASS;
assert_component!(PlanSummary);

/// The plan title over its move/unresolved counts.
#[component]
pub fn PlanSummary(props: PlanSummaryProps) -> Element {
    let counts = PlanCountsProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            PlanTitle {}
            PlanCounts { ..counts }
        }
    }
}
