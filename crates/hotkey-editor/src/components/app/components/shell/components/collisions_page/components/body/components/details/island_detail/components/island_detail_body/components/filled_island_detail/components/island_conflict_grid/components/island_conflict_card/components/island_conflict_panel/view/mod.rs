use super::super::super::presentation::{IslandAbilityData, IslandUnitData};

#[derive(Clone, PartialEq)]
pub struct IslandConflictPanelView {
    pub(crate) unit: IslandUnitData,
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

impl ddd::View for IslandConflictPanelView {}
