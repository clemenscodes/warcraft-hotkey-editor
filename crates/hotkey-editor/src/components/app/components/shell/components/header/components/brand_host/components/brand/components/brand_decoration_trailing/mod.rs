mod style;

use super::shared::brand_decoration_host::BrandDecorationHost;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The trailing flourish: the same decoration mirrored to face inward.
#[component]
pub fn BrandDecorationTrailing() -> Element {
    rsx! {
        span {
            class: CLASS,
            BrandDecorationHost {
            


            }
        }
    }
}

assert_component!(BrandDecorationTrailing);
