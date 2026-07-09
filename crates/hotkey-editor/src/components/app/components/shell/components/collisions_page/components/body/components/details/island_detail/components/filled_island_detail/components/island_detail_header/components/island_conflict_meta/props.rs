use super::super::super::IslandDetailHeaderProps;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The text meta column of the island detail header: the island's coordinate and its
/// collision count.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictMetaProps {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl From<&IslandDetailHeaderProps> for IslandConflictMetaProps {
    fn from(props: &IslandDetailHeaderProps) -> Self {
        let coordinate = props.coordinate;
        let count = props.count;
        Self { coordinate, count }
    }
}
