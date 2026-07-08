use crate::components::app::components::shell::components::shared::grid_tile::GridTileProps;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The already-built twelve read-only tiles the frame lays out. Each page shapes
/// its own tiles — placement icons on the resolve plan, one highlighted cell on the
/// collisions page — and hands the finished tiles to this shared frame, which owns
/// only the surrounding chrome and the tile-scoped border/radius overrides.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridFrameProps {
    pub tiles: [GridTileProps; COMMAND_GRID_TILE_COUNT],
}
