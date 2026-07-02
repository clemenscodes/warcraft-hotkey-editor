pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::plan_unresolved::PlanUnresolved;
use dioxus::prelude::*;
pub use props::PlanCountsProps;
use style::CLASS;
assert_component!(PlanCounts);

/// The move count (and, when present, the unresolved count) under the plan title.
#[component]
pub fn PlanCounts(props: PlanCountsProps) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    rsx! {
        span {
            class: CLASS,
            "{moves_text}"
            if unresolved_count > 0 {
                " · "
                PlanUnresolved { count: unresolved_count }
            }
        }
    }
}
