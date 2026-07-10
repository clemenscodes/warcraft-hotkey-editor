use super::super::super::super::super::logic::IslandAbilityData;
use dioxus::prelude::*;

/// The two abilities flanking the centered separator: the unit's own ability and the
/// ability it shares the key with.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictAbilityRowProps {
    pub own_ability: IslandAbilityData,
    pub shared_ability: IslandAbilityData,
}
