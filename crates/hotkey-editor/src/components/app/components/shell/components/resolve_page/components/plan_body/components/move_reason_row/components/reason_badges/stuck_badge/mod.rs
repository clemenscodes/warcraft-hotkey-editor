mod props;
use super::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::StuckBadgeProps;
use tw_macro::assert_component;
assert_component!(StuckBadge);

/// The "Stuck" reason badge: the base badge bound to the Orc colour.
#[component]
pub fn StuckBadge(props: StuckBadgeProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        ReasonBadge { ..badge }
    }
}
