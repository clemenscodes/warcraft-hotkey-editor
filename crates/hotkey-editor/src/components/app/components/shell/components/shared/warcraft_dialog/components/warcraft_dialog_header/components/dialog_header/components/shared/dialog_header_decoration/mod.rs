mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

#[component]
pub fn DialogHeaderDecoration() -> Element {
    rsx! {
        img {
            class: CLASS,
            src: HEADER_GOLD_DECORATION,
            alt: "",
            aria_hidden: "true",
        }
    }
}

assert_component!(DialogHeaderDecoration);
