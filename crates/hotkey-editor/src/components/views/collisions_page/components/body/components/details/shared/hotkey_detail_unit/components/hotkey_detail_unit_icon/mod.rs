mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::HotkeyDetailUnitIconProps;
use style::CLASS;
assert_component!(HotkeyDetailUnitIcon);
#[component]
pub fn HotkeyDetailUnitIcon(props: HotkeyDetailUnitIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
