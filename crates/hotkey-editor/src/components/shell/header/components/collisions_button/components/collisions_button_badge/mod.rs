mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::CollisionsButtonBadgeProps;

assert_component!(CollisionsButtonBadge);

#[component]
pub fn CollisionsButtonBadge(props: CollisionsButtonBadgeProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            "data-collision-badge": "true",
            aria_hidden: "true",
            {label}
        }
    }
}
