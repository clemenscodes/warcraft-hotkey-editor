mod props;
mod style;

use dioxus::prelude::*;
pub use props::BlockedDropTargetOverlayProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BlockedDropTargetOverlay);

/// The blocked-drop-target overlay. Mounted only when a drop here is refused; the
/// danger border and not-allowed cursor are the tile root's own (via
/// `:has(.blocked-drop-target-overlay)`), and this layer draws the danger wash.
#[component]
pub fn BlockedDropTargetOverlay(props: BlockedDropTargetOverlayProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}
