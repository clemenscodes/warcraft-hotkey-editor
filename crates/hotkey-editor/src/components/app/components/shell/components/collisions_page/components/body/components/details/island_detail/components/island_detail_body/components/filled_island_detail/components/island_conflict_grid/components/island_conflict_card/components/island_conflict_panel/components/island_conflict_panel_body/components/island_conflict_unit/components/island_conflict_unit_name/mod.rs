mod model;
mod view;

pub use view::IslandConflictUnitNameView;
mod style;
use dioxus::prelude::*;
use model::IslandConflictUnitNameModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn IslandConflictUnitName(props: IslandConflictUnitNameModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(IslandConflictUnitName);
