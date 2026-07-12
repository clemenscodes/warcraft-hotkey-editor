use super::super::super::presentation::{IslandAbilityData, IslandUnitData};

/// The published `View` contract mirroring [`IslandConflictPanelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictPanelView {
    pub(crate) unit: IslandUnitData,
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

impl ddd::View for IslandConflictPanelView {}
