mod model;
mod view;

pub use view::DragOverRingView;
mod style;

use dioxus::prelude::*;
use model::DragOverRingModel;
use style::CLASS;
use tw_macro::assert_component;

/// The drag-over marker. Mounted only while the cursor hovers this tile during a drag;
/// the gold border it produces is the tile root's own (via `:has(.drag-over-ring)`), so
/// this stays an inert, pointer-transparent presence signal. Shared by the filled and
/// empty tiles, which both nest it.
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
