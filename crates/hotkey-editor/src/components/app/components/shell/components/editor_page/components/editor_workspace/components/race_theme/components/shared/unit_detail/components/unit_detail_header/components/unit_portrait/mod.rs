mod props;
mod style;

use dioxus::prelude::*;
pub use props::UnitPortraitProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's portrait image; renders nothing when the unit has no portrait.
#[component]
pub fn UnitPortrait(props: UnitPortraitProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            loading: "lazy",
            decoding: "async",
        }
    }
}

assert_component!(UnitPortrait);
