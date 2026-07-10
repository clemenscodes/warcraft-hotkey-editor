pub mod components;
mod props;
mod style;
use components::plan_counts::PlanCounts;
use components::plan_title::PlanTitle;
use dioxus::prelude::*;
use props::PlanSummaryProps;
use style::CLASS;
use tw_macro::assert_component;

/// The plan title over its move/unresolved counts.
#[component]
pub fn PlanSummary(props: PlanSummaryProps) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    rsx! {
        div {
            class: CLASS,
            PlanTitle {}
            PlanCounts { moves_text, unresolved_count }
        }
    }
}

assert_component!(PlanSummary);
