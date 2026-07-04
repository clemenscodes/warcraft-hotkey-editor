mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::FollowerFigureProps;
use style::CLASS;
assert_component!(FollowerFigure);

#[component]
pub fn FollowerFigure(props: FollowerFigureProps) -> Element {
    let FollowerFigureProps { src, alt } = props;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            decoding: "async",
        }
    }
}
