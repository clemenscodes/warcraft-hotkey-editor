use super::view::PreviewHeadedGridView;
use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

#[derive(Props, Clone, PartialEq)]
pub struct PreviewHeadedGridModel {
    pub heading: &'static str,
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl From<&PreviewHeadedGridView> for PreviewHeadedGridModel {
    fn from(view: &PreviewHeadedGridView) -> Self {
        let PreviewHeadedGridView { heading, tiles } = view.clone();
        Self { heading, tiles }
    }
}

impl ddd::Model for PreviewHeadedGridModel {
    type View = PreviewHeadedGridView;
}
