use super::view::MiniGridView;
use crate::components::app::components::shell::components::shared::grid_tile::GridTileView;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

#[derive(Props, Clone, PartialEq)]
pub struct MiniGridModel {
    pub tiles: [GridTileView; COMMAND_GRID_TILE_COUNT],
}

impl From<&MiniGridView> for MiniGridModel {
    fn from(view: &MiniGridView) -> Self {
        let MiniGridView { tiles } = view.clone();
        Self { tiles }
    }
}

impl ddd::Model for MiniGridModel {
    type View = MiniGridView;
}
