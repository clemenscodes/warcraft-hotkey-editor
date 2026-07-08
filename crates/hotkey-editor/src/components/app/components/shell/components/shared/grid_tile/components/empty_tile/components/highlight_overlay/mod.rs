mod props;
mod style;

use dioxus::prelude::*;
pub use props::HighlightOverlayProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HighlightOverlay);

/// The mini-grid highlight overlay. Mounted only on the marked coordinate; the gold
/// border and glow are the tile root's own (via `:has(.highlight-overlay)`), and this
/// layer draws the gold wash.
#[component]
pub fn HighlightOverlay(props: HighlightOverlayProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}
