mod props;
mod style;

use dioxus::prelude::*;
pub use props::RegularCarrierBadgeProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(RegularCarrierBadge);

/// The carrier-count badge when its ability does not win the cell.
#[component]
pub fn RegularCarrierBadge(props: RegularCarrierBadgeProps) -> Element {
    let count = props.count;
    rsx! {
        span { class: CLASS, "{count}" }
    }
}
