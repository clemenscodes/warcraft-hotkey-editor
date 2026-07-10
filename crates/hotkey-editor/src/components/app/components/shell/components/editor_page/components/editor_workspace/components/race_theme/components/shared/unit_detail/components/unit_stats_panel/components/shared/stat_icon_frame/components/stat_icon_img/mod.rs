mod props;
mod style;

use dioxus::prelude::*;
pub use props::StatIconImgProps;
use style::CLASS;
use tw_macro::assert_component;

/// A stat column's icon image, filling its frame.
#[component]
pub fn StatIconImg(props: StatIconImgProps) -> Element {
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
