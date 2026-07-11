mod model;
mod view;

pub use view::ConflictAbilityNameView;
mod style;
use dioxus::prelude::*;
use model::ConflictAbilityNameModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictAbilityName(props: ConflictAbilityNameModel) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(ConflictAbilityName);
