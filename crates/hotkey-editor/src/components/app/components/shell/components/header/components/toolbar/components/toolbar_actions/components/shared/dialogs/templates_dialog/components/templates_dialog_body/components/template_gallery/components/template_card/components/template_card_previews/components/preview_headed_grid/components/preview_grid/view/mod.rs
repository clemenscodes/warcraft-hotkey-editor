use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

#[derive(Clone, PartialEq)]
pub struct PreviewGridView {
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for PreviewGridView {}
