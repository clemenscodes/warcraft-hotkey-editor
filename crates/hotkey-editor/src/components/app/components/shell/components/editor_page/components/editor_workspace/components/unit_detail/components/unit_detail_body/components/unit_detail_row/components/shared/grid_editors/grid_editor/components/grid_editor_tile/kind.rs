use super::{GridEditorTile, GridEditorTileProps};
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::GridTileKind;
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
