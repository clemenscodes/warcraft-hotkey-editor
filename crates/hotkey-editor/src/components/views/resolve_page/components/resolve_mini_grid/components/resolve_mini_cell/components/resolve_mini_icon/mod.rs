mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveMiniIconProps;
use style::CLASS;
assert_component!(ResolveMiniIcon);
#[component]
pub fn ResolveMiniIcon(props: ResolveMiniIconProps) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
