use super::components::editor_grid::components::grid_editor_tile::EditorTile;
use super::view::CaptionedEditorGridView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// A heading stacked above the editor grid: the caption plus the twelve finished
/// interactive tiles. `GridEditor` builds the tiles with their drag handlers in its
/// own reactive scope and hands them here; this carries no behavior of its own.
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
