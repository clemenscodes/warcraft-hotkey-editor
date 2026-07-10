use super::state::LayoutTileState;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// One editable grid cell as plain data: its visual state, the letter it shows, its
/// grid address, and the drag/click handlers the editor wired for it. The layout
/// editor builds these and threads them down to the grid, which renders one
/// [`LayoutTile`](super::LayoutTile) per view.
#[derive(Clone, PartialEq)]
pub struct LayoutTileView {
    pub state: LayoutTileState,
    pub label: String,
    pub coordinate: GridCoordinate,
    pub ondragstart: EventHandler<Event<DragData>>,
    pub ondragend: EventHandler<Event<DragData>>,
    pub ondragover: EventHandler<Event<DragData>>,
    pub ondrop: EventHandler<Event<DragData>>,
    pub onclick: EventHandler<MouseEvent>,
}
