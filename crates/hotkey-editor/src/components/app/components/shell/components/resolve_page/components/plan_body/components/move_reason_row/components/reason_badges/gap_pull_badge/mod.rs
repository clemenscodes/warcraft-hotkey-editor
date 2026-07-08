mod props;
use super::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::GapPullBadgeProps;
use tw_macro::assert_component;
assert_component!(GapPullBadge);

/// The "Gap pull" reason badge: the base badge bound to the Success colour.
#[component]
pub fn GapPullBadge(props: GapPullBadgeProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        ReasonBadge { ..badge }
    }
}
