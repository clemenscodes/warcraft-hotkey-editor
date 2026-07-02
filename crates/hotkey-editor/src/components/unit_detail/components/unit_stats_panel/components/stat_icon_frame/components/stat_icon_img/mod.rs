mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatIconImgProps;
use style::CLASS;
assert_component!(StatIconImg);

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
