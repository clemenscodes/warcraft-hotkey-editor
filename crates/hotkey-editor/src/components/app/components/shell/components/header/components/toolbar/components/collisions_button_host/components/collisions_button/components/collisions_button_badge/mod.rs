mod model;
mod view;

pub use view::CollisionsButtonBadgeView;
mod style;

use dioxus::prelude::*;
use model::CollisionsButtonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionsButtonBadge(props: CollisionsButtonBadgeModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            {label}
        }
    }
}

assert_component!(CollisionsButtonBadge);
