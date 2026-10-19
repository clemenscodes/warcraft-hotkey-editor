mod model;
mod view;

pub use view::HighlightOverlayView;
mod style;

use dioxus::prelude::*;
use model::HighlightOverlayModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HighlightOverlay(props: HighlightOverlayModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(HighlightOverlay);
