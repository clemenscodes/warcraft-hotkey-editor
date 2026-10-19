mod model;
mod view;

pub use view::SelectionRingView;
mod style;

use dioxus::prelude::*;
use model::SelectionRingModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SelectionRing(props: SelectionRingModel) -> Element {
    if !props.selected {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(SelectionRing);
