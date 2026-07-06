mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictAbilityNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictAbilityName);
#[component]
pub fn ConflictAbilityName(props: ConflictAbilityNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
