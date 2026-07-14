mod model;
mod view;

pub use view::UnitPortraitView;
mod style;

use dioxus::prelude::*;
use model::UnitPortraitModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitPortrait(props: UnitPortraitModel) -> Element {
    let src = props.src;
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
