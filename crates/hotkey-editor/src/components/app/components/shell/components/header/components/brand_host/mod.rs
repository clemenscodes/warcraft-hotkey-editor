pub mod components;
mod style;

use components::brand::Brand;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BrandHost() -> Element {
    rsx! {
        div {
            class: CLASS,
            Brand {



            }
        }
    }
}

assert_component!(BrandHost);
