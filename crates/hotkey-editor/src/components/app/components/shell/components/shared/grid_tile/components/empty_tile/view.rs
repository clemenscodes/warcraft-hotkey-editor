use super::super::super::GridTileState;

/// The published `View` contract mirroring [`EmptyTileProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EmptyTileView {
    pub state: GridTileState,
    /// True while the drag cursor hovers this drop target: it mounts the `DragOverRing`,
    /// and its dashed border turns gold.
    pub is_drag_over: bool,
}

impl ddd::View for EmptyTileView {}
