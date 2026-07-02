mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictAbilityNameProps;
use style::CLASS;
assert_component!(ConflictAbilityName);
#[component]
pub fn ConflictAbilityName(props: ConflictAbilityNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
