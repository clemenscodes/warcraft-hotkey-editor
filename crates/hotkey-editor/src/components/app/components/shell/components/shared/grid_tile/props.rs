use super::state::GridTileState;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, RowIndex};

/// Everything the base command tile needs to render. It is purely presentational
/// and entirely inert: it draws the look of its state and its icon or label; its race
/// accent is read from the inherited `--race-accent`. It has no hotkey, no focus, and no
/// event handlers — `GridEditorTile` layers all of that on top by wrapping this base
/// tile; the drag flags it forwards drive the mounted overlay children. Its address is
/// the domain `GridCoordinate`.
#[derive(Props, Clone, PartialEq)]
pub struct GridTileProps {
    /// The tile's address in the grid, the domain `GridCoordinate`.
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    /// Ability icon URL, drawn filling the tile when present.
    #[props(default)]
    pub icon: Option<String>,
    /// Shown centered when the tile has a label and no icon.
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub state: GridTileState,
    /// True while this tile is the lifted source of a drag. Drives the mounted
    /// `DraggingSourceGhost` on a filled tile; the read-only consumers leave it false.
    #[props(default)]
    pub is_dragging_source: bool,
    /// True while the drag cursor hovers this tile. Drives the mounted `DragOverRing`;
    /// the read-only consumers leave it false.
    #[props(default)]
    pub is_drag_over: bool,
}
