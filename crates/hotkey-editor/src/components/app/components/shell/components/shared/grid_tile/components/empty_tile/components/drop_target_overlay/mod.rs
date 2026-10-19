mod model;
mod view;

pub use view::DropTargetOverlayView;
mod style;

use dioxus::prelude::*;
use model::DropTargetOverlayModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DropTargetOverlay(props: DropTargetOverlayModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(DropTargetOverlay);
