mod props;
mod view;

pub use view::DropTargetOverlayView;
mod style;

use dioxus::prelude::*;
use props::DropTargetOverlayProps;
use style::CLASS;
use tw_macro::assert_component;

/// The drop-target marker overlay. Mounted only when the empty slot is the current
/// drop-target candidate; the dashed accent border is the tile root's own (via
/// `:has(.drop-target-overlay)`), so this stays an inert presence signal that may also
/// carry a parent-supplied under-cursor fill.
#[component]
pub fn DropTargetOverlay(props: DropTargetOverlayProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}

assert_component!(DropTargetOverlay);
