use super::view::MiniGridFrameView;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

#[derive(Props, Clone, PartialEq)]
pub struct MiniGridFrameModel {
    pub tiles: [GridTileView; COMMAND_GRID_TILE_COUNT],
}

impl From<&MiniGridFrameView> for MiniGridFrameModel {
    fn from(view: &MiniGridFrameView) -> Self {
        let MiniGridFrameView { tiles } = view.clone();
        Self { tiles }
    }
}

impl ddd::Model for MiniGridFrameModel {
    type View = MiniGridFrameView;
}
