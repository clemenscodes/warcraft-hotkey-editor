mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::HotkeyUnitNameProps;
use style::CLASS;
assert_component!(HotkeyUnitName);
/// A unit's name on a collision card.
#[component]
pub fn HotkeyUnitName(props: HotkeyUnitNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
