mod props;
use super::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::SwapBadgeProps;
use tw_macro::assert_component;
assert_component!(SwapBadge);

/// The "Swap" reason badge: the base badge bound to the Undead colour.
#[component]
pub fn SwapBadge(props: SwapBadgeProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        ReasonBadge { ..badge }
    }
}
