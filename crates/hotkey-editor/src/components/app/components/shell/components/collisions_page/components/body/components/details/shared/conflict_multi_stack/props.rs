use super::super::conflict_card_model::ConflictAbilityData;
use super::super::conflict_marker_view::ConflictMarker;
use dioxus::prelude::*;

/// The multi-way clash layout: the conflict marker stacked above every clashing
/// ability, or nothing when the clash is a two-ability pair.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMultiStackProps {
    pub abilities: Vec<ConflictAbilityData>,
    pub marker: ConflictMarker,
}
