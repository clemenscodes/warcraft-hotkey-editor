use super::super::super::logic::{IslandAbilityData, IslandUnitData};
use super::view::IslandConflictPanelView;
use dioxus::prelude::*;

/// The card surface: the affected unit heading the two clashing abilities that flank
/// the centered separator.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictPanelProps {
    pub(crate) unit: IslandUnitData,
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

impl From<&IslandConflictPanelView> for IslandConflictPanelProps {
    fn from(view: &IslandConflictPanelView) -> Self {
        let IslandConflictPanelView {
            unit,
            own_ability,
            shared_ability,
        } = view.clone();
        Self {
            unit,
            own_ability,
            shared_ability,
        }
    }
}

impl ddd::Props for IslandConflictPanelProps {
    type View = IslandConflictPanelView;
}
