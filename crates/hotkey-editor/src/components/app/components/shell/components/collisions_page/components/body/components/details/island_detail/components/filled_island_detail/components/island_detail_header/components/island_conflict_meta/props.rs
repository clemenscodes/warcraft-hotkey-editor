use super::view::IslandConflictMetaView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The text meta column of the island detail header: the island's coordinate and its
/// collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictMetaProps {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl From<&IslandConflictMetaView> for IslandConflictMetaProps {
    fn from(view: &IslandConflictMetaView) -> Self {
        let IslandConflictMetaView { coordinate, count } = view.clone();
        Self { coordinate, count }
    }
}

impl ddd::Props for IslandConflictMetaProps {
    type View = IslandConflictMetaView;
}
