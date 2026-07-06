pub mod components;
mod props;
mod style;
use components::plan_unresolved::PlanUnresolved;
use dioxus::prelude::*;
pub use props::PlanCountsProps;
use style::CLASS;
use tw_macro::assert_component;
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
