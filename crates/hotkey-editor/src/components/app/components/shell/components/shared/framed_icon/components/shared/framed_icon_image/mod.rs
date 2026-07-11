mod model;
mod view;

pub use view::FramedIconImageView;
mod style;

use dioxus::prelude::*;
use model::FramedIconImageModel;
use style::CLASS;
use tw_macro::assert_component;

/// The image inside a `FramedIcon`: it fills the frame and covers it, scaling with
/// the slot its parent hands it rather than pinning its own size. Presentational:
/// source and alt in, markup out.
#[component]
pub fn FramedIconImage(props: FramedIconImageModel) -> Element {
    let source = props.source;
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src: source,
            alt,
            loading: "lazy",
            decoding: "async",
        }
    }
}

assert_component!(FramedIconImage);
