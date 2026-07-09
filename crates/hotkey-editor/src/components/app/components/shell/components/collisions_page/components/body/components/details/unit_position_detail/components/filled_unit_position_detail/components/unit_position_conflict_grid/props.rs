use super::components::unit_position_conflict_card::UnitPositionConflictCardProps;
use dioxus::prelude::*;

/// The scrolling grid of position-collision cards for the selected unit.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPositionConflictGridProps {
    pub cards: Vec<UnitPositionConflictCardProps>,
}
