mod style;

use super::shared::brand_decoration_host::BrandDecorationHost;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The leading flourish: the decoration facing inward as drawn.
#[component]
pub fn BrandDecorationLeading() -> Element {
    rsx! {
        span { class: CLASS, BrandDecorationHost {} }
    }
}

assert_component!(BrandDecorationLeading);
