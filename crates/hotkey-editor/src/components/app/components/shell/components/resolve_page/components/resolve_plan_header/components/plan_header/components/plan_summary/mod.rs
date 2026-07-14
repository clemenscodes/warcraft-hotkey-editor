pub mod components;
mod model;
mod view;

pub use view::PlanSummaryView;
mod style;
use components::plan_counts::PlanCounts;
use components::plan_title::PlanTitle;
use dioxus::prelude::*;
use model::PlanSummaryModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PlanSummary(props: PlanSummaryModel) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    rsx! {
        div {
            class: CLASS,
            PlanTitle {}
            PlanCounts {
                moves_text,
                unresolved_count,
            }
        }
    }
}

assert_component!(PlanSummary);
