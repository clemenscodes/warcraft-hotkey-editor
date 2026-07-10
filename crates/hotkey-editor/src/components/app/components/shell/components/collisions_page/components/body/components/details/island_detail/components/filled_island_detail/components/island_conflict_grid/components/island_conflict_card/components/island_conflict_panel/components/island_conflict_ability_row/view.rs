use super::super::super::super::super::logic::IslandAbilityData;

/// The published `View` contract mirroring [`IslandConflictAbilityRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictAbilityRowView {
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

impl ddd::View for IslandConflictAbilityRowView {}
