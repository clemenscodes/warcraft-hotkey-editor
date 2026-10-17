use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

#[derive(Clone, PartialEq)]
pub struct PreviewGridView {
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl ddd::Layered for PreviewGridView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PreviewGridView {}
