mod model;
mod view;

pub use view::StatIconImgView;
mod style;

use dioxus::prelude::*;
use model::StatIconImgModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn StatIconImg(props: StatIconImgModel) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
        }
    }
}

assert_component!(StatIconImg);
