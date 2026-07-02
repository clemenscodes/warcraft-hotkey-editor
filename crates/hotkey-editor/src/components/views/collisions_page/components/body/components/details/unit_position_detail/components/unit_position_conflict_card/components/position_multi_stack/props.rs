use super::super::conflict_position_cell::ConflictPositionCellProps;
use crate::components::views::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbilityProps;
use dioxus::prelude::*;

/// The 3+-way clash layout: the colliding cell stacked above every landing ability,
/// or nothing when the clash is an exact pair.
#[derive(Props, Clone, PartialEq)]
pub struct PositionMultiStackProps {
    pub abilities: Vec<ConflictAbilityProps>,
    pub cell: ConflictPositionCellProps,
}
