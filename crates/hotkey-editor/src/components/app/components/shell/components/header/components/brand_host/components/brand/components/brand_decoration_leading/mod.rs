mod style;

use super::shared::brand_decoration_host::BrandDecorationHost;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BrandDecorationLeading() -> Element {
    rsx! {
        span {
            class: CLASS,
            BrandDecorationHost {}
        }
    }
}

assert_component!(BrandDecorationLeading);
