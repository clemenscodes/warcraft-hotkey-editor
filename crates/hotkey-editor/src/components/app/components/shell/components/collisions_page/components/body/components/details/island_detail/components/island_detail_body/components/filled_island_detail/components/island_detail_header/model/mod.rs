use super::view::IslandDetailHeaderView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The island detail pane header: the island's coordinate (its mini-grid and coordinate
/// line) and its collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailHeaderModel {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl From<&IslandDetailHeaderView> for IslandDetailHeaderModel {
    fn from(view: &IslandDetailHeaderView) -> Self {
        let IslandDetailHeaderView { coordinate, count } = view.clone();
        Self { coordinate, count }
    }
}

impl ddd::Model for IslandDetailHeaderModel {
    type View = IslandDetailHeaderView;
}
