mod model;
mod view;

pub use view::AbilityFillView;
mod style;

use dioxus::prelude::*;
use model::AbilityFillModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AbilityFill(props: AbilityFillModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(AbilityFill);
