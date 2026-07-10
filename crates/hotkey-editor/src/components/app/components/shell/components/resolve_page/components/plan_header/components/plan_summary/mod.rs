pub mod components;
mod props;
mod style;
use components::plan_counts::{PlanCounts, PlanCountsProps};
use components::plan_title::PlanTitle;
use dioxus::prelude::*;
pub use props::PlanSummaryProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(PlanSummary);
