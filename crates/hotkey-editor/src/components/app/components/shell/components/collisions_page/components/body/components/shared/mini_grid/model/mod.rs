use super::view::MiniGridView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct MiniGridModel {
    pub coordinate: GridCoordinate,
}

impl From<&MiniGridView> for MiniGridModel {
    fn from(view: &MiniGridView) -> Self {
        let MiniGridView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Model for MiniGridModel {
    type View = MiniGridView;
}
