use super::components::editor_grid::components::grid_editor_tile::EditorTile;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The published `View` contract mirroring [`EditorHeadedGridModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EditorHeadedGridView {
    pub heading: &'static str,
    pub(crate) tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for EditorHeadedGridView {}
