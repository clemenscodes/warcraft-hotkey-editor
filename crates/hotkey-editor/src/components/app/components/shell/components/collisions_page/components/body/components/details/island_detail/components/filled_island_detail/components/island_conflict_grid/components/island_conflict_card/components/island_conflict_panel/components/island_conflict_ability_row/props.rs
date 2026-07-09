use super::components::island_conflict_ability::IslandConflictAbilityProps;
use dioxus::prelude::*;

/// The two abilities flanking the centered separator: the unit's own ability and the
/// ability it shares the key with.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictAbilityRowProps {
    pub own_ability: IslandConflictAbilityProps,
    pub shared_ability: IslandConflictAbilityProps,
}
