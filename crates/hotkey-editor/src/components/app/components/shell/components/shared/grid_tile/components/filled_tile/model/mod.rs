use super::super::super::GridTileState;
use super::view::FilledTileView;
use dioxus::prelude::*;

/// An occupied command tile: the ability/command icon (or its text fallback). Its
/// background, selection ring, dragging-source ghost and drag-over ring are each a
/// conditionally-mounted child, and `FilledTilePresentation` shapes which ones mount from the
/// slot's `GridTileState` — so the tile root stays one mounted element across every
/// state.
#[derive(Props, Clone, PartialEq)]
pub struct FilledTileModel {
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

impl From<&FilledTileView> for FilledTileModel {
    fn from(view: &FilledTileView) -> Self {
        let FilledTileView {
            state,
            icon,
            label,
            is_dragging_source,
            is_drag_over,
        } = view.clone();
        Self {
            state,
            icon,
            label,
            is_dragging_source,
            is_drag_over,
        }
    }
}

impl ddd::Model for FilledTileModel {
    type View = FilledTileView;
}
