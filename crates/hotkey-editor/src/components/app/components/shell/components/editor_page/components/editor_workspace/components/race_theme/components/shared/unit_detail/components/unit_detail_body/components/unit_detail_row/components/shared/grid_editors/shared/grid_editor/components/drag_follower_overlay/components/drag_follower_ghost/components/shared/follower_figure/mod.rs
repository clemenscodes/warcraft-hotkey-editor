mod model;
mod view;

pub use view::FollowerFigureView;
mod style;

use dioxus::prelude::*;
use model::FollowerFigureModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FollowerFigure(props: FollowerFigureModel) -> Element {
    let FollowerFigureModel { src, alt } = props;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            decoding: "async",
        }
    }
}

assert_component!(FollowerFigure);
