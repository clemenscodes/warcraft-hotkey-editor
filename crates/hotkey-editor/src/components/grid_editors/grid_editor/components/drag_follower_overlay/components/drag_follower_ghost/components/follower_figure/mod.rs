mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::FollowerFigureProps;

assert_component!(FollowerFigure);

#[component]
pub fn FollowerFigure(props: FollowerFigureProps) -> Element {
    let FollowerFigureProps { src, alt } = props;
    rsx! {
        img { class: CLASS, src, alt, decoding: "async" }
    }
}
