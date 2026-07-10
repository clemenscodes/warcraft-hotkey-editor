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
