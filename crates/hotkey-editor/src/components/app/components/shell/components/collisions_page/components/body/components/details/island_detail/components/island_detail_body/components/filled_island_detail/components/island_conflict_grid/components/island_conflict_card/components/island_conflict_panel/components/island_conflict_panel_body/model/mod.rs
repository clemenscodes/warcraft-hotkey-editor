use super::super::super::super::super::presentation::{IslandAbilityData, IslandUnitData};
use super::view::IslandConflictPanelBodyView;
use dioxus::prelude::*;

/// One island conflict card's data: the affected unit heading it, and its two clashing
/// abilities. Carried by the body region so it can shape a single card inside the shared
/// `PanelCard` surface.
#[derive(Clone, PartialEq)]
pub struct IslandConflictCardData {
    pub(crate) unit: IslandUnitData,
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

/// The island conflict panel card's body region input: the card data carried as a list so the
/// region is `Default`-able. Exactly one card is present in practice; the body renders the
/// affected unit over its clashing-abilities row.
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
