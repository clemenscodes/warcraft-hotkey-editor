mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::HotkeyUnitRowIconProps;
use style::CLASS;
assert_component!(HotkeyUnitRowIcon);
/// A unit's portrait on a collision card.
#[component]
pub fn HotkeyUnitRowIcon(props: HotkeyUnitRowIconProps) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! { img { class: CLASS, src, alt, loading: "lazy", decoding: "async" } }
}
