pub mod components;
mod props;
mod state;
mod style;

use super::shared::drag_over_ring::{DragOverRing, DragOverRingProps};
use components::blocked_drop_target_overlay::{
    BlockedDropTargetOverlay, BlockedDropTargetOverlayProps,
};
use components::drop_target_overlay::{DropTargetOverlay, DropTargetOverlayProps};
use components::highlight_overlay::{HighlightOverlay, HighlightOverlayProps};
use dioxus::prelude::*;
pub use props::EmptyTileProps;
pub use state::EmptyTileState;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EmptyTile);

/// An empty command slot. Purely presentational: it draws its resting look and its race
/// accent, and — during a drag or a mini-grid highlight — mounts the matching overlay
/// child (drop-target / blocked / highlight, plus the `DragOverRing` under the cursor)
/// whose presence turns the tile's own border into that look. It knows nothing of
/// hotkeys, focus, or dragging; `GridEditorTile` layers all interaction on top.
#[component]
pub fn EmptyTile(props: EmptyTileProps) -> Element {
    let drop_target = DropTargetOverlayProps::from(&props);
    let blocked_drop_target = BlockedDropTargetOverlayProps::from(&props);
    let highlight = HighlightOverlayProps::from(&props);
    let drag_over_ring = DragOverRingProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            DropTargetOverlay { ..drop_target }
            BlockedDropTargetOverlay { ..blocked_drop_target }
            HighlightOverlay { ..highlight }
            DragOverRing { ..drag_over_ring }
        }
    }
}
