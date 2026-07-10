use crate::components::app::components::shell::components::shared::grid_tile::GridTileView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The read-only mini grid's input: the twelve inert base tiles to lay out, always
/// exactly `COMMAND_GRID_TILE_COUNT` of them.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridProps {
    pub tiles: [GridTileView; COMMAND_GRID_TILE_COUNT],
}
