pub mod components;
mod props;
mod style;
use components::resolve_apply_button::{ResolveApplyButton, ResolveApplyButtonProps};
use components::resolve_plan_summary::{ResolvePlanSummary, ResolvePlanSummaryProps};
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolvePlanHeaderProps;
use style::CLASS;
assert_component!(ResolvePlanHeader);

/// The plan header row: cascade summary on the left, Apply button on the right.
#[component]
pub fn ResolvePlanHeader(props: ResolvePlanHeaderProps) -> Element {
    let summary = ResolvePlanSummaryProps::from(&props);
    let apply = ResolveApplyButtonProps::from(&props);
    rsx! {
        header {
            class: CLASS,
            ResolvePlanSummary { ..summary }
            ResolveApplyButton { ..apply }
        }
    }
}
