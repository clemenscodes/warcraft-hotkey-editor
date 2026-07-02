use super::{GridTile, GridTileProps};
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::GridTileKind;
use dioxus::prelude::*;

/// The [`GridTileKind`] that binds the generic `Grid` to the inert base tile. A
/// `Grid<PlainTileKind>` is a read-only grid of plain `GridTile`s — no hotkeys, no
/// interaction — which the mini grids reuse to draw the command grid's shape with
/// one coordinate highlighted.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct PlainTileKind;

impl GridTileKind for PlainTileKind {
    type Tile = GridTileProps;

    fn tile(tile: Self::Tile) -> Element {
        rsx! {
            GridTile { ..tile }
        }
    }
}
