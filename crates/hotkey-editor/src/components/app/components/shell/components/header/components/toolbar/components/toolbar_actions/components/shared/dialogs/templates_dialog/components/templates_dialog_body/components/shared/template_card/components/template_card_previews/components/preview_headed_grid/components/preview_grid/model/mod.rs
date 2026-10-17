use super::view::PreviewGridView;
use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

#[derive(Props, Clone, PartialEq)]
pub struct PreviewGridModel {
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl From<&PreviewGridView> for PreviewGridModel {
    fn from(view: &PreviewGridView) -> Self {
        let PreviewGridView { tiles } = view.clone();
        Self { tiles }
    }
}

impl ddd::Layered for PreviewGridModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PreviewGridModel {
    type View = PreviewGridView;
}
