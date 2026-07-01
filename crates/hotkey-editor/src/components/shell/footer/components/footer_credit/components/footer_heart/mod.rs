mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::FooterHeartProps;

assert_component!(FooterHeart);

#[component]
pub fn FooterHeart(props: FooterHeartProps) -> Element {
    let svg = props.svg;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: svg,
        }
    }
}
