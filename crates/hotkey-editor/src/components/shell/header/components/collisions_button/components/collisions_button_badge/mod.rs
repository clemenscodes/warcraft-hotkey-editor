mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionsButtonBadgeProps;
use style::CLASS;
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
