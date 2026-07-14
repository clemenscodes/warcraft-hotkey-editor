pub mod components;
mod data;
mod presentation;
mod style;

use components::brand_decoration_leading::BrandDecorationLeading;
use components::brand_decoration_trailing::BrandDecorationTrailing;
use components::brand_title::BrandTitle;
use dioxus::prelude::*;
use presentation::use_brand;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn Brand() -> Element {
    let onclick = use_brand();
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-label": "Warcraft III Hotkey Editor \u{2014} return to editor",
            onclick,
            BrandDecorationLeading {



            }
            BrandTitle {
                title: data::TITLE,
            }
            BrandDecorationTrailing {



            }
        }
    }
}

assert_component!(Brand);
