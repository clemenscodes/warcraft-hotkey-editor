use super::super::super::logic::IslandDetailData;
use super::components::island_conflict_grid::IslandConflictGridProps;
use super::components::island_conflict_grid::components::island_conflict_card::IslandConflictCardProps;
use super::components::island_detail_header::IslandDetailHeaderProps;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The populated island detail pane: the island's coordinate, its collision count, and
/// its per-unit conflict cards.
#[derive(Props, Clone, PartialEq)]
pub struct FilledIslandDetailProps {
    pub coordinate: GridCoordinate,
    pub count: usize,
    pub cards: Vec<IslandConflictCardProps>,
}

impl From<&IslandDetailData> for FilledIslandDetailProps {
    fn from(data: &IslandDetailData) -> Self {
        let coordinate = data.coordinate;
        let count = data.count;
        let cards = data.cards.clone();
        Self {
            coordinate,
            count,
            cards,
        }
    }
}

impl From<&FilledIslandDetailProps> for IslandDetailHeaderProps {
    fn from(props: &FilledIslandDetailProps) -> Self {
        let coordinate = props.coordinate;
        let count = props.count;
        Self { coordinate, count }
    }
}

impl From<&FilledIslandDetailProps> for IslandConflictGridProps {
    fn from(props: &FilledIslandDetailProps) -> Self {
        let cards = props.cards.clone();
        Self { cards }
    }
}
