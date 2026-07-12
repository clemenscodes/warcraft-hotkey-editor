use super::view::IslandConflictMetaView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The text meta column of the island detail header: the island's coordinate and its
/// collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictMetaModel {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl From<&IslandConflictMetaView> for IslandConflictMetaModel {
    fn from(view: &IslandConflictMetaView) -> Self {
        let IslandConflictMetaView { coordinate, count } = view.clone();
        Self { coordinate, count }
    }
}

impl ddd::Model for IslandConflictMetaModel {
    type View = IslandConflictMetaView;
}
