mod model;
mod view;

pub use view::PagerCardPortraitView;
mod style;

use dioxus::prelude::*;
use model::PagerCardPortraitModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardPortrait(props: PagerCardPortraitModel) -> Element {
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

assert_component!(PagerCardPortrait);
