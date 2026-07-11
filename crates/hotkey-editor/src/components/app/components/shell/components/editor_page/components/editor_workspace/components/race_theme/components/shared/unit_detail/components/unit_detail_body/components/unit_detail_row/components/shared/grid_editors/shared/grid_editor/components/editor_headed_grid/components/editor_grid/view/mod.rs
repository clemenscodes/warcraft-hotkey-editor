use super::components::grid_editor_tile::EditorTile;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The published `View` contract mirroring [`EditorGridModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EditorGridView {
    pub(crate) tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for EditorGridView {}
