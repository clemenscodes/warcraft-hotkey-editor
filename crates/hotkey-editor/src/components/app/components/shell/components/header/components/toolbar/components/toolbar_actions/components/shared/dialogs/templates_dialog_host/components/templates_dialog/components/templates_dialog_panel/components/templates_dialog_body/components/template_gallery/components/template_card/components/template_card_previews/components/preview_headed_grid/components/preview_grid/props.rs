use super::view::PreviewGridView;
use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile};

/// The read-only preview grid's input: the twelve resolved domain tiles to lay out,
/// always exactly `COMMAND_GRID_TILE_COUNT` of them. Each is adapted to the shared
/// `TileFace` painter's fields at render time.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewGridProps {
    pub tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

impl From<&PreviewGridView> for PreviewGridProps {
    fn from(view: &PreviewGridView) -> Self {
        let PreviewGridView { tiles } = view.clone();
        Self { tiles }
    }
}

impl ddd::Props for PreviewGridProps {
    type View = PreviewGridView;
}
