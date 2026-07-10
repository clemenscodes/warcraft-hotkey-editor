use super::super::super::GridTileState;

/// The published `View` contract mirroring [`FilledTileProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FilledTileView {
    pub state: GridTileState,
    /// The ability icon URL, drawn filling the tile when present.
    pub icon: Option<String>,
    /// Shown centered when the tile has a label and no icon.
    pub label: String,
    /// True while this tile is the lifted source of a drag: it mounts the
    /// `DraggingSourceGhost`, and its root turns into the dashed deep-blue ghost.
    pub is_dragging_source: bool,
    /// True while the drag cursor hovers this tile: it mounts the `DragOverRing`, and
    /// its root's border turns gold.
    pub is_drag_over: bool,
}

impl ddd::View for FilledTileView {}
