use super::view::MiniGridFrameView;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The already-built twelve read-only tiles the frame lays out. Each page shapes
/// its own tiles — placement icons on the resolve plan, one highlighted cell on the
/// collisions page — and hands the finished tiles to this shared frame, which owns
/// only the surrounding chrome and the tile-scoped border/radius overrides.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridFrameProps {
    pub tiles: [GridTileView; COMMAND_GRID_TILE_COUNT],
}

impl From<&MiniGridFrameView> for MiniGridFrameProps {
    fn from(view: &MiniGridFrameView) -> Self {
        let MiniGridFrameView { tiles } = view.clone();
        Self { tiles }
    }
}

impl ddd::Props for MiniGridFrameProps {
    type View = MiniGridFrameView;
}
