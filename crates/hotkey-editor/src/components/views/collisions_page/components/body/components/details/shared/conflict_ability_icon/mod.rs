mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictAbilityIconProps;
use style::CLASS;
assert_component!(ConflictAbilityIcon);
#[component]
pub fn ConflictAbilityIcon(props: ConflictAbilityIconProps) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
