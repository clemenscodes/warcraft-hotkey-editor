mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictUnitNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictUnitName);
/// A unit's name on a collision card.
#[component]
pub fn ConflictUnitName(props: ConflictUnitNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
