mod model;
mod view;

pub use view::UnitDetailPortraitView;
mod style;

use dioxus::prelude::*;
use model::UnitDetailPortraitModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailPortrait(props: UnitDetailPortraitModel) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    rsx! {
        img {
            class: CLASS,
            src,
            alt: "",
            loading: "lazy",
            decoding: "async",
        }
    }
}

assert_component!(UnitDetailPortrait);
