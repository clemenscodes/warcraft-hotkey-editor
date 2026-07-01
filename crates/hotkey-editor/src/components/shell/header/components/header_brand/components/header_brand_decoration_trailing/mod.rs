mod style;

use dioxus::prelude::*;

use super::header_brand_decoration::HeaderBrandDecoration;
use crate::assert_component;
use style::CLASS;

assert_component!(HeaderBrandDecorationTrailing);

/// The trailing flourish: the same decoration mirrored to face inward.
#[component]
pub fn HeaderBrandDecorationTrailing() -> Element {
    rsx! {
        span {
            class: CLASS,
            HeaderBrandDecoration {}
        }
    }
}
