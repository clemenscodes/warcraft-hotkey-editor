mod model;
mod view;

pub use view::FooterHeartView;
mod style;

use dioxus::prelude::*;
use model::FooterHeartModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterHeart(props: FooterHeartModel) -> Element {
    let svg = props.svg;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: svg,
        }
    }
}

assert_component!(FooterHeart);
