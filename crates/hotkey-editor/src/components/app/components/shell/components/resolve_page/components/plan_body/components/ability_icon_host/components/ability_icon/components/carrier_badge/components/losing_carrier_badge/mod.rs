mod model;
mod view;

pub use view::LosingCarrierBadgeView;
mod style;

use dioxus::prelude::*;
use model::LosingCarrierBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn LosingCarrierBadge(props: LosingCarrierBadgeModel) -> Element {
    let count = props.count;
    rsx! {
        span {
            class: CLASS,
            "{count}"
        }
    }
}

assert_component!(LosingCarrierBadge);
