mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::DraggableMarkerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DraggableMarker(props: DraggableMarkerModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(DraggableMarker);
