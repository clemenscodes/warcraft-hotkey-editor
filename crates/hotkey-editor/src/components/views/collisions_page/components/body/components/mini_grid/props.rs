use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// A tiny 4×3 command grid highlighting the given coordinate's cell.
#[derive(Props, Clone, PartialEq)]
pub struct MiniGridProps {
    pub coordinate: GridCoordinate,
}
