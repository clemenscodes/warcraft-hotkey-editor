use super::components::island_conflict_ability_row::IslandConflictAbilityRowProps;
use super::components::island_conflict_ability_row::components::island_conflict_ability::IslandConflictAbilityProps;
use super::components::island_conflict_unit::IslandConflictUnitProps;
use dioxus::prelude::*;

/// The card surface: the affected unit heading the two clashing abilities that flank
/// the centered separator.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictPanelProps {
    pub unit: IslandConflictUnitProps,
    pub own_ability: IslandConflictAbilityProps,
    pub shared_ability: IslandConflictAbilityProps,
}

impl From<&IslandConflictPanelProps> for IslandConflictAbilityRowProps {
    fn from(props: &IslandConflictPanelProps) -> Self {
        let own_ability = props.own_ability.clone();
        let shared_ability = props.shared_ability.clone();
        Self {
            own_ability,
            shared_ability,
        }
    }
}
