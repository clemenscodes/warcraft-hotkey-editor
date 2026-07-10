use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The text meta column of the island detail header: the island's coordinate and its
/// collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictMetaProps {
    pub coordinate: GridCoordinate,
    pub count: usize,
}
