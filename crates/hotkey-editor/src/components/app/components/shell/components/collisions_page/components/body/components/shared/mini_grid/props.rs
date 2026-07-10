use super::view::MiniGridView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// A tiny 4×3 command grid highlighting the given coordinate's cell.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridProps {
    pub coordinate: GridCoordinate,
}

impl From<&MiniGridView> for MiniGridProps {
    fn from(view: &MiniGridView) -> Self {
        let MiniGridView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Props for MiniGridProps {
    type View = MiniGridView;
}
