use crate::components::app::components::shell::components::shared::grid_tile::GridTileView;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub tiles: [GridTileView; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for MiniGridView {}
