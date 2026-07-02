mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::FightIconProps;
use style::CLASS;
assert_component!(FightIcon);
#[component]
pub fn FightIcon(props: FightIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
