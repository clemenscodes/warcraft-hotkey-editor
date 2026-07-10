pub mod components;
mod props;
mod style;
use components::apply_button::{ApplyButton, ApplyButtonProps};
use components::plan_summary::{PlanSummary, PlanSummaryProps};
use dioxus::prelude::*;
pub use props::PlanHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

/// The plan header row: cascade summary on the left, Apply button on the right.
#[component]
pub fn PlanHeader(props: PlanHeaderProps) -> Element {
    let summary = PlanSummaryProps::from(&props);
    let apply = ApplyButtonProps::from(&props);
    rsx! {
        header {
            class: CLASS,
            PlanSummary { ..summary }
            ApplyButton { ..apply }
        }
    }
}

assert_component!(PlanHeader);
