use super::view::PreviewHeadedGridView;
use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

/// A heading stacked above a read-only preview grid: the caption plus the twelve
/// resolved domain tiles. Carries no behavior — the templates preview builds the tiles
/// and hands them here.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewHeadedGridProps {
    pub heading: &'static str,
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl From<&PreviewHeadedGridView> for PreviewHeadedGridProps {
    fn from(view: &PreviewHeadedGridView) -> Self {
        let PreviewHeadedGridView { heading, tiles } = view.clone();
        Self { heading, tiles }
    }
}

impl ddd::Props for PreviewHeadedGridProps {
    type View = PreviewHeadedGridView;
}
