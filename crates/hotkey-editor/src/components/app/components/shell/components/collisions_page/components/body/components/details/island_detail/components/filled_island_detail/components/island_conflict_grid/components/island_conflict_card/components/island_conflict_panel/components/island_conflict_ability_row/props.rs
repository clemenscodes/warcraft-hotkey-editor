use super::super::super::super::super::logic::IslandAbilityData;
use super::view::IslandConflictAbilityRowView;
use dioxus::prelude::*;

/// The two abilities flanking the centered separator: the unit's own ability and the
/// ability it shares the key with.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictAbilityRowProps {
    pub(crate) own_ability: IslandAbilityData,
    pub(crate) shared_ability: IslandAbilityData,
}

impl From<&IslandConflictAbilityRowView> for IslandConflictAbilityRowProps {
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

impl ddd::Props for IslandConflictAbilityRowProps {
    type View = IslandConflictAbilityRowView;
}
