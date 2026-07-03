mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionsButtonIconProps;
use style::CLASS;
assert_component!(CollisionsButtonIcon);

#[component]
pub fn CollisionsButtonIcon(props: CollisionsButtonIconProps) -> Element {
    let svg = props.svg;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: svg }
    }
}
