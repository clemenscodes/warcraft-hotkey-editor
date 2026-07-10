use super::super::super::GridTileState;
use dioxus::prelude::*;

/// An empty command slot. During a drag it mounts an overlay child that makes it a drop
/// target (or a blocked one), or a mini-grid highlight, and — when the cursor is over it
/// — the `DragOverRing`. `EmptyTileModel` shapes which overlay mounts from the slot's
/// `GridTileState`; the overlays are what the tile root's own border keys off, so no
/// look-flag attribute lives on the root.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyTileProps {
    pub state: GridTileState,
    /// True while the drag cursor hovers this drop target: it mounts the `DragOverRing`,
    /// and its dashed border turns gold.
    pub is_drag_over: bool,
}
