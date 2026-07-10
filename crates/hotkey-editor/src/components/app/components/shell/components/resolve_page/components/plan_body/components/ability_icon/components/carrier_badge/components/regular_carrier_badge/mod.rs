mod props;
mod view;

pub use view::RegularCarrierBadgeView;
mod style;

use dioxus::prelude::*;
use props::RegularCarrierBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The carrier-count badge when its ability does not win the cell.
#[component]
pub fn RegularCarrierBadge(props: RegularCarrierBadgeProps) -> Element {
    let count = props.count;
    rsx! {
        span { class: CLASS, "{count}" }
    }
}

assert_component!(RegularCarrierBadge);
