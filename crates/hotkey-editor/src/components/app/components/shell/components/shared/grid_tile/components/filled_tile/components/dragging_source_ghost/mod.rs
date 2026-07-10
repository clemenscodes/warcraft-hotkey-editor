mod props;
mod style;

use dioxus::prelude::*;
use props::DraggingSourceGhostProps;
use style::CLASS;
use tw_macro::assert_component;

/// The dragging-source ghost. Mounted only while this tile is the lifted drag source;
/// the dashed deep-blue ghost border is the tile root's own (via
/// `:has(.dragging-source-ghost)`), so this layer only draws the opaque panel that
/// hides the lifted icon, and stays pointer-transparent so the source remains
/// hit-testable as its own drop target.
#[component]
pub fn DraggingSourceGhost(props: DraggingSourceGhostProps) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div { class: CLASS }
    }
}

assert_component!(DraggingSourceGhost);
