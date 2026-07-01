mod style;

use super::header_brand_decoration::HeaderBrandDecoration;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(HeaderBrandDecorationLeading);

/// The leading flourish: the decoration facing inward as drawn.
#[component]
pub fn HeaderBrandDecorationLeading() -> Element {
    rsx! {
        span { class: CLASS, HeaderBrandDecoration {} }
    }
}
