mod model;
mod view;

pub use view::DraggingSourceGhostView;
mod style;

use dioxus::prelude::*;
use model::DraggingSourceGhostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DraggingSourceGhost(props: DraggingSourceGhostModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(DraggingSourceGhost);
