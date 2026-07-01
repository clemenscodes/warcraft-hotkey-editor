mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::UnitPortraitProps;
use style::CLASS;
assert_component!(UnitPortrait);

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
