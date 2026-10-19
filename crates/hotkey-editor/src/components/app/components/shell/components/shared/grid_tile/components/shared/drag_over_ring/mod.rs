mod model;
mod view;

pub use view::DragOverRingView;
mod style;

use dioxus::prelude::*;
use model::DragOverRingModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DragOverRing(props: DragOverRingModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(DragOverRing);
