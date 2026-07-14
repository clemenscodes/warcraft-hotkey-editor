mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::DraggableMarkerModel;
use style::CLASS;
use tw_macro::assert_component;

/// The draggable marker. Mounted only when the tile may be dragged; the grab cursor is
/// the tile root's own (via `:has(.draggable-marker)`), so this stays an inert,
/// pointer-transparent presence signal the root and the off-state picker key off.
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
