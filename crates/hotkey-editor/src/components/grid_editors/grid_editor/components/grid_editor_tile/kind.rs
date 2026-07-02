use super::{GridEditorTile, GridEditorTileProps};
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::GridTileKind;
use dioxus::prelude::*;

/// The [`GridTileKind`] that binds the generic `Grid` to the interactive editor
/// tile. The editor's grids are `Grid<EditorTileKind>`, so every cell is a
/// `GridEditorTile`.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct EditorTileKind;

impl GridTileKind for EditorTileKind {
    type Tile = GridEditorTileProps;

    fn tile(tile: Self::Tile) -> Element {
        rsx! {
            GridEditorTile { ..tile }
        }
    }
}
