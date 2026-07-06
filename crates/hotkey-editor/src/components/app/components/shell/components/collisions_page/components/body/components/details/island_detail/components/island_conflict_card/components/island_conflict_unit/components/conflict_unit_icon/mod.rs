mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictUnitIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictUnitIcon);
#[component]
pub fn ConflictUnitIcon(props: ConflictUnitIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
