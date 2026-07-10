use super::super::super::logic::{IslandAbilityData, IslandUnitData};
use dioxus::prelude::*;

/// The card surface: the affected unit heading the two clashing abilities that flank
/// the centered separator.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictPanelProps {
    pub unit: IslandUnitData,
    pub own_ability: IslandAbilityData,
    pub shared_ability: IslandAbilityData,
}
