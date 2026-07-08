use crate::components::app::components::shell::components::shared::tile_face::TileFaceProps;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The read-only preview grid's input: the twelve resolved `TileFace` painters to
/// lay out, always exactly `COMMAND_GRID_TILE_COUNT` of them.
#[derive(Props, Clone, PartialEq)]
pub struct PreviewGridProps {
    pub tiles: [TileFaceProps; COMMAND_GRID_TILE_COUNT],
}
