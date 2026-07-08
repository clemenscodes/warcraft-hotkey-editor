use super::components::grid_editor_tile::GridEditorTileProps;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The interactive grid's input: the twelve finished editor tiles to lay out,
/// always exactly `COMMAND_GRID_TILE_COUNT` of them. Each tile carries its own
/// interaction; `EditorGrid` only arranges them.
#[derive(Props, Clone, PartialEq)]
pub struct EditorGridProps {
    pub tiles: [GridEditorTileProps; COMMAND_GRID_TILE_COUNT],
}
