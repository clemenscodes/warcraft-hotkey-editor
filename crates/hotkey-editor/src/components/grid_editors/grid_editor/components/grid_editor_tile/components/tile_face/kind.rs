use super::{TileFace, TileFaceProps};
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::GridTileKind;
use dioxus::prelude::*;

/// The read-only [`GridTileKind`]: a `Grid<TileFaceKind>` paints each cell as a
/// `TileFace` — icon plus hotkey badge — with no interaction. The templates preview
/// and the gallery use it to draw the command grid's tiles without the editor's
/// drag/select behavior. The interactive editor grid uses `EditorTileKind` instead.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct TileFaceKind;

impl GridTileKind for TileFaceKind {
    type Tile = TileFaceProps;

    fn tile(tile: Self::Tile) -> Element {
        rsx! {
            TileFace { ..tile }
        }
    }
}
