mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CarrierCardIconProps;
use style::CLASS;
assert_component!(CarrierCardIcon);
#[component]
pub fn CarrierCardIcon(props: CarrierCardIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
