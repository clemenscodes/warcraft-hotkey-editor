use crate::components::app::components::shell::components::shared::grid_tile::GridTileView;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The published `View` contract mirroring [`MiniGridProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub tiles: [GridTileView; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for MiniGridView {}
