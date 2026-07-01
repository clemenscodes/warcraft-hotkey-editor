mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveFightIconProps;
use style::CLASS;
assert_component!(ResolveFightIcon);
#[component]
pub fn ResolveFightIcon(props: ResolveFightIconProps) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
