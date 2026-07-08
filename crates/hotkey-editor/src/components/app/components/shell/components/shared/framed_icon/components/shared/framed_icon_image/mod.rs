mod props;
mod style;

use dioxus::prelude::*;
pub use props::FramedIconImageProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FramedIconImage);

/// The image inside a `FramedIcon`: it fills the frame and covers it, scaling with
/// the slot its parent hands it rather than pinning its own size. Presentational:
/// source and alt in, markup out.
#[component]
pub fn FramedIconImage(props: FramedIconImageProps) -> Element {
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
