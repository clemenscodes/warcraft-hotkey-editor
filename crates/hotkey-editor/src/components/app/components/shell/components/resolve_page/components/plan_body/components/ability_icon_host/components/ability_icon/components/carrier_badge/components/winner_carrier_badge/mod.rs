mod model;
mod view;

pub use view::WinnerCarrierBadgeView;
mod style;

use dioxus::prelude::*;
use model::WinnerCarrierBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn WinnerCarrierBadge(props: WinnerCarrierBadgeModel) -> Element {
    let count = props.count;
    rsx! {
        span {
            class: CLASS,
            "{count}"
        }
    }
}

assert_component!(WinnerCarrierBadge);
