pub mod components;
mod style;

use components::brand_decoration::BrandDecoration;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BrandDecorationHost() -> Element {
    rsx! {
        div {
            class: CLASS,
            BrandDecoration {}
        }
    }
}

assert_component!(BrandDecorationHost);
