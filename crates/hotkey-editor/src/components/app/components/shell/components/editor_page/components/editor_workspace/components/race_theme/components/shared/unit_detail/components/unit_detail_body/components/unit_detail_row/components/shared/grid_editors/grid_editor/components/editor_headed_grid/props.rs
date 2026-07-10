use super::components::editor_grid::components::grid_editor_tile::EditorTile;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// A heading stacked above the editor grid: the caption plus the twelve finished
/// interactive tiles. `GridEditor` builds the tiles with their drag handlers in its
/// own reactive scope and hands them here; this carries no behavior of its own.
#[derive(Props, Clone, PartialEq)]
pub struct EditorHeadedGridProps {
    pub heading: &'static str,
    pub tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
}
