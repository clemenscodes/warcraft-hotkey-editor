mod model;
mod view;

pub use view::ReasonBadgeView;
mod style;

use dioxus::prelude::*;
use model::ReasonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ReasonBadge(props: ReasonBadgeModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(ReasonBadge);
