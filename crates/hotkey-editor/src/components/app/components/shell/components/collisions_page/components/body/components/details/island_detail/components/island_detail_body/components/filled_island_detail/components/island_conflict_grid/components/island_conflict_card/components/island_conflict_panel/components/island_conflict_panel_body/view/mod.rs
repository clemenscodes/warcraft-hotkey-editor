use super::super::super::super::super::presentation::{IslandAbilityData, IslandUnitData};
use super::IslandConflictPanelBody;
use super::model::IslandConflictPanelBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IslandConflictCardData {
    pub(crate) unit: IslandUnitData,
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

#[derive(Clone, PartialEq, Default)]
pub struct IslandConflictPanelBodyView {
    pub(crate) cards: Vec<IslandConflictCardData>,
}

impl ddd::Layered for IslandConflictPanelBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandConflictPanelBodyView {}

impl Render for IslandConflictPanelBodyView {
    type Model = IslandConflictPanelBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let cards = self.cards.clone();
        rsx! {
            IslandConflictPanelBody {
                cards,
            }
        }
    }
}
