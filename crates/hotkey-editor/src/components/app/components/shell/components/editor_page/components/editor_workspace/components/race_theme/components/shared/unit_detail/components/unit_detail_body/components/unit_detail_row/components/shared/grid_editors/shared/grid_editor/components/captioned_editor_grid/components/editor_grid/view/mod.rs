use super::components::grid_editor_tile::EditorTile;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

#[derive(Clone, PartialEq)]
pub struct EditorGridView {
    pub(crate) tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for EditorGridView {}
