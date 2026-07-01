pub mod components;
mod data;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use components::header_brand_decoration_leading::HeaderBrandDecorationLeading;
use components::header_brand_decoration_trailing::HeaderBrandDecorationTrailing;
use components::header_brand_title::HeaderBrandTitle;

pub use props::HeaderBrandProps;

assert_component!(HeaderBrand);

/// The app's wordmark: the title flanked by mirrored gold flourishes, the whole
/// thing a button that returns to the editor. The flourishes show in both the
/// compact and full header layouts.
#[component]
pub fn HeaderBrand(props: HeaderBrandProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "aria-label": "Warcraft III Hotkey Editor \u{2014} return to editor",
            "data-action": "view-editor",
            onclick,
            HeaderBrandDecorationLeading {}
            HeaderBrandTitle { ..data::TITLE }
            HeaderBrandDecorationTrailing {}
        }
    }
}
