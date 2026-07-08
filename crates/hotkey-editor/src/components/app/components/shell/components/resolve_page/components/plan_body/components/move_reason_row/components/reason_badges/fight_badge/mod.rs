mod props;
use super::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::FightBadgeProps;
use tw_macro::assert_component;
assert_component!(FightBadge);

/// The "Fight" reason badge: the base badge bound to the Orc colour.
#[component]
pub fn FightBadge(props: FightBadgeProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        ReasonBadge { ..badge }
    }
}
