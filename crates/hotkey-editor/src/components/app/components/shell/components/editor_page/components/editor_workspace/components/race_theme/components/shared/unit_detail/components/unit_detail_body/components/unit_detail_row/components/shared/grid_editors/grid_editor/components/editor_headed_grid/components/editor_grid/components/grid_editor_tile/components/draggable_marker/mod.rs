mod props;
mod style;

use dioxus::prelude::*;
pub use props::DraggableMarkerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DraggableMarker);

/// The draggable marker. Mounted only when the tile may be dragged; the grab cursor is
/// the tile root's own (via `:has(.draggable-marker)`), so this stays an inert,
/// pointer-transparent presence signal the root and the off-state picker key off.
#[component]
pub fn DraggableMarker(props: DraggableMarkerProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}
