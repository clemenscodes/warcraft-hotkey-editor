use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The island detail pane header: the island's coordinate (its mini-grid and coordinate
/// line) and its collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailHeaderProps {
    pub coordinate: GridCoordinate,
    pub count: usize,
}
