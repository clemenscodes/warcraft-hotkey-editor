mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

/// The gold flourish itself. It scales to fill its host container via container-query
/// units and keeps its aspect ratio, so it never distorts; the host owns the box and
/// the leading/trailing wrappers orient it.
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

assert_component!(BrandDecoration);
