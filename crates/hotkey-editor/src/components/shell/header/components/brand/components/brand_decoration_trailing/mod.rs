mod style;

use super::shared::brand_decoration::BrandDecoration;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(BrandDecorationTrailing);

/// The trailing flourish: the same decoration mirrored to face inward.
#[component]
pub fn BrandDecorationTrailing() -> Element {
    rsx! {
        span { class: CLASS, BrandDecoration {} }
    }
}
