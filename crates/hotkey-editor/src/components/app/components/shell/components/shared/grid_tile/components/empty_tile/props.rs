use super::super::super::GridTileProps;
use super::super::super::GridTileState;
use super::state::EmptyTileState;
use dioxus::prelude::*;

/// An empty command slot. During a drag it mounts an overlay child that makes it a drop
/// target (or a blocked one), or a mini-grid highlight, and — when the cursor is over it
/// — the `DragOverRing`. The overlays are what the tile root's own border keys off, so
/// no look-flag attribute lives on the root.
#[derive(Props, Clone, PartialEq)]
pub struct EmptyTileProps {
    pub state: EmptyTileState,
    /// True while the drag cursor hovers this drop target: it mounts the `DragOverRing`,
    /// and its dashed border turns gold.
    pub is_drag_over: bool,
}

impl From<&GridTileProps> for EmptyTileProps {
    fn from(props: &GridTileProps) -> Self {
        let state = match props.state {
            GridTileState::DropTarget => EmptyTileState::DropTarget,
            GridTileState::BlockedDropTarget => EmptyTileState::BlockedDropTarget,
            GridTileState::Highlighted => EmptyTileState::Highlighted,
            _ => EmptyTileState::Empty,
        };
        let is_drag_over = props.is_drag_over;
        Self {
            state,
            is_drag_over,
        }
    }
}
