mod model;
mod view;

pub use view::BlockedDropTargetOverlayView;
mod style;

use dioxus::prelude::*;
use model::BlockedDropTargetOverlayModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BlockedDropTargetOverlay(props: BlockedDropTargetOverlayModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(BlockedDropTargetOverlay);
