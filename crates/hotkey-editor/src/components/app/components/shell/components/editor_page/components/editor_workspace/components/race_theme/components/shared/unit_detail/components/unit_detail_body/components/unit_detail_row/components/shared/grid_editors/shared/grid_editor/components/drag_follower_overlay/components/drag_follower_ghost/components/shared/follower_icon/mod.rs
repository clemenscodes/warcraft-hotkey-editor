mod model;
mod view;

pub use view::FollowerIconView;
mod style;

use dioxus::prelude::*;
use model::FollowerIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FollowerIcon(props: FollowerIconModel) -> Element {
    let FollowerIconModel { src, alt } = props;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            decoding: "async",
        }
    }
}

assert_component!(FollowerIcon);
