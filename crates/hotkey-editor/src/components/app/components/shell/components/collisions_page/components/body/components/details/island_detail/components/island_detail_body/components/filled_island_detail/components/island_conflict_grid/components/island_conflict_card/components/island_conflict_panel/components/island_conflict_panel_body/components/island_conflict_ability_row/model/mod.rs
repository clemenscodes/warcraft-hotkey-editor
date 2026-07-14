use super::super::super::super::super::super::super::presentation::IslandAbilityData;
use super::view::IslandConflictAbilityRowView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictAbilityRowModel {
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

impl From<&IslandConflictAbilityRowView> for IslandConflictAbilityRowModel {
    fn from(view: &IslandConflictAbilityRowView) -> Self {
        let IslandConflictAbilityRowView {
            own_ability,
            shared_ability,
        } = view.clone();
        Self {
            own_ability,
            shared_ability,
        }
    }
}

impl ddd::Model for IslandConflictAbilityRowModel {
    type View = IslandConflictAbilityRowView;
}
