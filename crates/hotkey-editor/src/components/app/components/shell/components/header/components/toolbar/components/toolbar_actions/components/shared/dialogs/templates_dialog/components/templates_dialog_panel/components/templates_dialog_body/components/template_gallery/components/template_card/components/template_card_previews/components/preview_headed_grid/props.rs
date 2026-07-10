use super::components::preview_grid::PreviewGridProps;
use crate::components::app::components::shell::components::shared::tile_face::TileFaceProps;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// A heading stacked above a read-only preview grid: the caption plus the twelve
/// resolved painters. Carries no behavior — the templates preview builds the tiles
/// and hands them here.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewHeadedGridProps {
    pub heading: &'static str,
    pub tiles: [TileFaceProps; COMMAND_GRID_TILE_COUNT],
}

impl From<&PreviewHeadedGridProps> for PreviewGridProps {
    fn from(props: &PreviewHeadedGridProps) -> Self {
        let tiles = props.tiles.clone();
        Self { tiles }
    }
}
