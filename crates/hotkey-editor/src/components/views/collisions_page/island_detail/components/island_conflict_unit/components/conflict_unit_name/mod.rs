mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictUnitNameProps;
use style::CLASS;
assert_component!(ConflictUnitName);
#[component]
pub fn ConflictUnitName(props: ConflictUnitNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
