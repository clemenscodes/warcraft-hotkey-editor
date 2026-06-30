use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

use super::components::grid_tile::GridTileProps;

/// The grid's only input: the finished tiles to draw. Always exactly
/// `COMMAND_GRID_TILE_COUNT` of them, a hard domain invariant (the command grid
/// is forever three rows by four columns), so the type is a fixed-size array, not
/// a slice. The grid is a pure presentational engine: it lays the tiles out and
/// nothing else. All behavior, drag, selection, moves, is built by the owning
/// `GridEditor` and baked into each `GridTileProps`, so a read-only consumer can
/// hand it plain tiles.
#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    pub tiles: [GridTileProps; COMMAND_GRID_TILE_COUNT],
}
