mod model;
mod view;

pub use view::ConflictUnitNameView;
mod style;
use dioxus::prelude::*;
use model::ConflictUnitNameModel;
use style::CLASS;
use tw_macro::assert_component;
/// A unit's name on a collision card.
#[component]
pub fn ConflictUnitName(props: ConflictUnitNameModel) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(ConflictUnitName);
