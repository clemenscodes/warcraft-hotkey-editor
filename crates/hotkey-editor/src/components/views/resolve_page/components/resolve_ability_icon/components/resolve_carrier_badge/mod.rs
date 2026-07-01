mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveCarrierBadgeProps;
use style::CLASS;
assert_component!(ResolveCarrierBadge);
#[component]
pub fn ResolveCarrierBadge(props: ResolveCarrierBadgeProps) -> Element {
    let count = props.count;
    let is_winner = props.is_winner;
    rsx! { span { class: CLASS, "data-win": is_winner, "{count}" } }
}
