mod props;
mod style;

use dioxus::prelude::*;
pub use props::FollowerFigureProps;
use style::CLASS;
use tw_macro::assert_component;
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
