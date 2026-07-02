mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CarrierBadgeProps;
use style::CLASS;
assert_component!(CarrierBadge);
#[component]
pub fn CarrierBadge(props: CarrierBadgeProps) -> Element {
    let count = props.count;
    let is_winner = props.is_winner;
    rsx! { span { class: CLASS, "data-win": is_winner, "{count}" } }
}
