mod model;
mod view;

pub use view::FilterTriggerBadgeView;
mod style;

use dioxus::prelude::*;
use model::FilterTriggerBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilterTriggerBadge(props: FilterTriggerBadgeModel) -> Element {
    let count = props.count;
    rsx! {
        span {
            class: CLASS,
            "{count}"
        }
    }
}

assert_component!(FilterTriggerBadge);
