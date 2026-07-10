mod props;
mod style;

use dioxus::prelude::*;
pub use props::DragOverRingProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DragOverRing);

/// The drag-over marker. Mounted only while the cursor hovers this tile during a drag;
/// the gold border it produces is the tile root's own (via `:has(.drag-over-ring)`), so
/// this stays an inert, pointer-transparent presence signal. Shared by the filled and
/// empty tiles, which both nest it.
#[component]
pub fn DragOverRing(props: DragOverRingProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}
