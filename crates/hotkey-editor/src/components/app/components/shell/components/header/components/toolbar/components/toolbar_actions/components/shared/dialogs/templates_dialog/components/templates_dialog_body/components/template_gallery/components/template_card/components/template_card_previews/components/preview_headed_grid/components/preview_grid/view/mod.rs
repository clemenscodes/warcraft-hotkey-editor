use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

/// The published `View` contract mirroring [`PreviewGridModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PreviewGridView {
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::View for PreviewGridView {}
