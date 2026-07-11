mod model;
mod view;

pub use view::HighlightOverlayView;
mod style;

use dioxus::prelude::*;
use model::HighlightOverlayModel;
use style::CLASS;
use tw_macro::assert_component;

/// The mini-grid highlight overlay. Mounted only on the marked coordinate; the gold
/// border and glow are the tile root's own (via `:has(.highlight-overlay)`), and this
/// layer draws the gold wash.
#[component]
pub fn HighlightOverlay(props: HighlightOverlayModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}

assert_component!(HighlightOverlay);
