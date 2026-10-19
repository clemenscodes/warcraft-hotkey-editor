mod model;
mod view;

pub use view::TileIconView;
mod style;

use dioxus::prelude::*;
use model::TileIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TileIcon(props: TileIconModel) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            ondragstart: move |event| event.prevent_default(),
            loading: "lazy",
            decoding: "async",
        }
    }
}

assert_component!(TileIcon);
