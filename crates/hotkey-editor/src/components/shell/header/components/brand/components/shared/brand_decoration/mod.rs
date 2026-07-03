mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");
assert_component!(BrandDecoration);

/// The gold flourish itself, sized for the header. The leading and trailing
/// wrappers orient it; this base never flips.
#[component]
pub fn BrandDecoration() -> Element {
    rsx! {
        img {
            class: CLASS,
            src: HEADER_GOLD_DECORATION,
            alt: "",
            aria_hidden: "true",
        }
    }
}
