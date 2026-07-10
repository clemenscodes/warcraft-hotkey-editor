mod props;
mod style;

use dioxus::prelude::*;
use props::CollisionsButtonBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionsButtonBadge(props: CollisionsButtonBadgeProps) -> Element {
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
