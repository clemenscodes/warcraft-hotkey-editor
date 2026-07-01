mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictUnitIconProps;
use style::CLASS;
assert_component!(ConflictUnitIcon);
#[component]
pub fn ConflictUnitIcon(props: ConflictUnitIconProps) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
