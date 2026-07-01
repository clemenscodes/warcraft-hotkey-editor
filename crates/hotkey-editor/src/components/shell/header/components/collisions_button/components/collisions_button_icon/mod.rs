mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::CollisionsButtonIconProps;

assert_component!(CollisionsButtonIcon);

#[component]
pub fn CollisionsButtonIcon(props: CollisionsButtonIconProps) -> Element {
    let svg = props.svg;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: svg,
        }
    }
}
