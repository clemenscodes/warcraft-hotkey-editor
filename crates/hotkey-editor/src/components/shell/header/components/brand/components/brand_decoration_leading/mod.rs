mod style;

use super::shared::brand_decoration::BrandDecoration;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(BrandDecorationLeading);

/// The leading flourish: the decoration facing inward as drawn.
#[component]
pub fn BrandDecorationLeading() -> Element {
    rsx! {
        span { class: CLASS, BrandDecoration {} }
    }
}
