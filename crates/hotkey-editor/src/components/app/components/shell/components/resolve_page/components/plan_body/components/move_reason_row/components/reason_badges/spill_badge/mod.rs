mod props;
use super::reason_badge::{ReasonBadge, ReasonBadgeProps};
use dioxus::prelude::*;
pub use props::SpillBadgeProps;
use tw_macro::assert_component;
assert_component!(SpillBadge);

/// The "Spill" reason badge: the base badge bound to the Human colour.
#[component]
pub fn SpillBadge(props: SpillBadgeProps) -> Element {
    let badge = ReasonBadgeProps::from(&props);
    rsx! {
        ReasonBadge { ..badge }
    }
}
