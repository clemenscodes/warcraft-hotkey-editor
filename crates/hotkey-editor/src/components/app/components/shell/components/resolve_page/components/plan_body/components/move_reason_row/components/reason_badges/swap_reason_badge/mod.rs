mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::components::reason_badges::shared::reason_badge::{
    ReasonBadge, ReasonBadgeProps,
};
use dioxus::prelude::*;
pub use props::SwapReasonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SwapReasonBadge);

/// The "Swap" reason badge: publishes the Swap accent colour and composes the base
/// `ReasonBadge` pill with the domain label. It adds only its colour on top and never
/// names the pill's classes.
#[component]
pub fn SwapReasonBadge(props: SwapReasonBadgeProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        span {
            class: CLASS,
            ReasonBadge { ..badge }
        }
    }
}
