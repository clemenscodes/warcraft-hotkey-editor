use super::components::editor_grid::components::grid_editor_tile::EditorTile;
use super::view::CaptionedEditorGridView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

#[derive(Props, Clone, PartialEq)]
pub struct CaptionedEditorGridModel {
    pub heading: &'static str,
    pub(crate) tiles: [EditorTile; COMMAND_GRID_TILE_COUNT],
}

impl From<&CaptionedEditorGridView> for CaptionedEditorGridModel {
    fn from(view: &CaptionedEditorGridView) -> Self {
        let CaptionedEditorGridView { heading, tiles } = view.clone();
        Self { heading, tiles }
    }
}

impl ddd::Model for CaptionedEditorGridModel {
    type View = CaptionedEditorGridView;
}
