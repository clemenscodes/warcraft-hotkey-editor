use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

#[derive(Clone, PartialEq)]
pub struct PreviewHeadedGridView {
    pub heading: &'static str,
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for PreviewHeadedGridView {}
