use super::super::conflict_position_cell::ConflictPositionCellProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbilityProps;
use dioxus::prelude::*;

/// The two abilities flanking the colliding cell in a pair clash, with the cell.
#[derive(Clone, PartialEq)]
pub struct PositionPair {
    pub left: ConflictAbilityProps,
    pub right: ConflictAbilityProps,
    pub cell: ConflictPositionCellProps,
}

/// The pair-clash row: two abilities flanking the colliding cell, or nothing when
/// the clash is not an exact pair.
#[derive(Props, Clone, PartialEq)]
pub struct PositionPairRowProps {
    pub pair: Option<PositionPair>,
}
