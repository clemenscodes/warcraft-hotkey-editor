mod model;
mod view;

pub use view::SwapReasonBadgeView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_badge::components::reason_badges::shared::reason_badge::ReasonBadge;
use dioxus::prelude::*;
use model::SwapReasonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

/// The "Swap" reason badge: publishes the Swap accent colour and composes the base
/// `ReasonBadge` pill with the domain label. It adds only its colour on top and never
/// names the pill's classes.
#[component]
pub fn SwapReasonBadge(props: SwapReasonBadgeModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ReasonBadge {
                label,
            }
        }
    }
}

assert_component!(SwapReasonBadge);
