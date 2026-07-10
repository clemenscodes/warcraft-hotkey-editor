mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::components::reason_badges::shared::reason_badge::ReasonBadge;
use dioxus::prelude::*;
use props::StuckReasonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The "Stuck" reason badge: publishes the Stuck accent colour and composes the base
/// `ReasonBadge` pill with the domain label. It adds only its colour on top and never
/// names the pill's classes.
#[component]
pub fn StuckReasonBadge(props: StuckReasonBadgeProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ReasonBadge { label }
        }
    }
}

assert_component!(StuckReasonBadge);
