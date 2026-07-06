mod props;
mod style;
use dioxus::prelude::*;
pub use props::HotkeyUnitNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyUnitName);
/// A unit's name on a collision card.
#[component]
pub fn HotkeyUnitName(props: HotkeyUnitNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
