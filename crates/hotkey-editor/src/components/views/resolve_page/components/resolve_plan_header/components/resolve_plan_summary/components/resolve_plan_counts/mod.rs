pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::resolve_plan_unresolved::ResolvePlanUnresolved;
use dioxus::prelude::*;
pub use props::ResolvePlanCountsProps;
use style::CLASS;
assert_component!(ResolvePlanCounts);

/// The move count (and, when present, the unresolved count) under the plan title.
#[component]
pub fn ResolvePlanCounts(props: ResolvePlanCountsProps) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    rsx! {
        span {
            class: CLASS,
            "{moves_text}"
            if unresolved_count > 0 {
                " · "
                ResolvePlanUnresolved { count: unresolved_count }
            }
        }
    }
}
