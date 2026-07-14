use super::components::grid_editor_tile::EditorTile;
use super::view::EditorGridView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

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
