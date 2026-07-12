use super::components::grid_editor_tile::EditorTile;
use super::view::EditorGridView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The interactive grid's input: the twelve finished editor tiles to lay out,
/// always exactly `COMMAND_GRID_TILE_COUNT` of them. Each tile carries its own
/// interaction; `EditorGrid` only arranges them.
#[derive(Props, Clone, PartialEq)]
pub struct EditorGridModel {
    pub(crate) tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
}

impl From<&EditorGridView> for EditorGridModel {
    fn from(view: &EditorGridView) -> Self {
        let EditorGridView { tiles } = view.clone();
        Self { tiles }
    }
}

impl ddd::Model for EditorGridModel {
    type View = EditorGridView;
}
