use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

use super::components::grid::components::grid_tile::GridTileProps;

/// A heading stacked above a grid of finished tiles. Purely presentational: it
/// pairs a caption with the reused `Grid` and draws whatever tiles it is handed.
/// It has no behavior of its own. The `GridEditor` builds these props with
/// interactive tiles, the templates preview builds them with read-only tiles;
/// either way `HeadedGrid` just renders.
#[derive(Props, Clone, PartialEq)]
pub struct HeadedGridProps {
    pub heading: &'static str,
    pub tiles: [GridTileProps; COMMAND_GRID_TILE_COUNT],
}
