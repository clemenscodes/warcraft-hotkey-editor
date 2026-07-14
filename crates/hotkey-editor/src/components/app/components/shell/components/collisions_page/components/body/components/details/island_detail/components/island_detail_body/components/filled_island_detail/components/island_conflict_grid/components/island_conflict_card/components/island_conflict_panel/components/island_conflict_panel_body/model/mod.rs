use super::super::super::super::super::presentation::{IslandAbilityData, IslandUnitData};
use super::view::IslandConflictPanelBodyView;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IslandConflictCardData {
    pub(crate) unit: IslandUnitData,
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictPanelBodyModel {
    pub(crate) cards: Vec<IslandConflictCardData>,
}

impl From<&IslandConflictPanelBodyView> for IslandConflictPanelBodyModel {
    fn from(view: &IslandConflictPanelBodyView) -> Self {
        let IslandConflictPanelBodyView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Model for IslandConflictPanelBodyModel {
    type View = IslandConflictPanelBodyView;
}
