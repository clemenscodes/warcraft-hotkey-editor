mod props;
mod view;

pub use view::ConflictAbilityNameView;
mod style;
use dioxus::prelude::*;
use props::ConflictAbilityNameProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictAbilityName(props: ConflictAbilityNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(ConflictAbilityName);
