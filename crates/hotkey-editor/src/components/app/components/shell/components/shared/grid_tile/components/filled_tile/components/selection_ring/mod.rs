mod model;
mod view;

pub use view::SelectionRingView;
mod style;

use dioxus::prelude::*;
use model::SelectionRingModel;
use style::CLASS;
use tw_macro::assert_component;

/// The selected-tile marker. Mounted only when the tile is selected; the gold border
/// and glow are the tile root's own (driven by `:has(.selection-ring)` in its style),
/// so this stays an inert presence signal that both the root look and the shell's
/// scroll/focus coordinator key off.
#[component]
pub fn SelectionRing(props: SelectionRingModel) -> Element {
    if !props.selected {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}

assert_component!(SelectionRing);
