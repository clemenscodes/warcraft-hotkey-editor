mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::MiniIconProps;
use style::CLASS;
assert_component!(MiniIcon);
#[component]
pub fn MiniIcon(props: MiniIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
