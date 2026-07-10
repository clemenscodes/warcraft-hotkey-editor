mod props;
mod style;

use dioxus::prelude::*;
pub use props::FooterHeartProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterHeart(props: FooterHeartProps) -> Element {
    let svg = props.svg;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: svg }
    }
}

assert_component!(FooterHeart);
