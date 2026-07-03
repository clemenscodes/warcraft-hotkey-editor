pub mod components;
mod data;
mod hooks;
mod style;

use crate::assert_component;
use components::brand_decoration_leading::BrandDecorationLeading;
use components::brand_decoration_trailing::BrandDecorationTrailing;
use components::brand_title::BrandTitle;
use dioxus::prelude::*;
use hooks::use_brand;
use style::CLASS;
assert_component!(Brand);

/// The app's wordmark: the title flanked by mirrored gold flourishes, the whole
/// thing a button that returns to the editor. It wires its own return-to-editor
/// click from the navigation context; the header only places it. The flourishes
/// show in both the compact and full header layouts.
#[component]
pub fn Brand() -> Element {
    let onclick = use_brand();
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-label": "Warcraft III Hotkey Editor \u{2014} return to editor",
            "data-action": "view-editor",
            onclick,
            BrandDecorationLeading {}
            BrandTitle { ..data::TITLE }
            BrandDecorationTrailing {}
        }
    }
}
