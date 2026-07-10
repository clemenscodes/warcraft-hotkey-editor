mod props;
mod style;

use dioxus::prelude::*;
pub use props::WinnerCarrierBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

/// The carrier-count badge when its ability wins the cell: gold.
#[component]
pub fn WinnerCarrierBadge(props: WinnerCarrierBadgeProps) -> Element {
    let count = props.count;
    rsx! {
        span { class: CLASS, "{count}" }
    }
}

assert_component!(WinnerCarrierBadge);
