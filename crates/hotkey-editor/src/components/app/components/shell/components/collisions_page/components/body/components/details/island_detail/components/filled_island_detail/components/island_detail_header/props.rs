use super::view::IslandDetailHeaderView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The island detail pane header: the island's coordinate (its mini-grid and coordinate
/// line) and its collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailHeaderProps {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl From<&IslandDetailHeaderView> for IslandDetailHeaderProps {
    fn from(view: &IslandDetailHeaderView) -> Self {
        let IslandDetailHeaderView { coordinate, count } = view.clone();
        Self { coordinate, count }
    }
}

impl ddd::Props for IslandDetailHeaderProps {
    type View = IslandDetailHeaderView;
}
