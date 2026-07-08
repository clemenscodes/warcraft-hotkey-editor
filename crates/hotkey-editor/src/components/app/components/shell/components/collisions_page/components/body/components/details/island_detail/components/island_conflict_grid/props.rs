use super::components::island_conflict_card::IslandConflictCardProps;
use dioxus::prelude::*;

/// The scrolling grid of conflict cards for the selected island.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictGridProps {
    pub cards: Vec<IslandConflictCardProps>,
}
