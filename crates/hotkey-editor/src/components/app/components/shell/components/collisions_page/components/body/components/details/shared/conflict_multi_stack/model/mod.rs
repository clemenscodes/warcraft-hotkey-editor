use super::super::conflict_card_model::ConflictAbilityData;
use super::super::conflict_marker_view::ConflictMarker;
use super::view::ConflictMultiStackView;
use dioxus::prelude::*;

/// The multi-way clash layout: the conflict marker stacked above every clashing
/// ability, or nothing when the clash is a two-ability pair.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMultiStackModel {
    pub(crate) abilities: Vec<ConflictAbilityData>,
    pub marker: ConflictMarker,
}

impl From<&ConflictMultiStackView> for ConflictMultiStackModel {
    fn from(view: &ConflictMultiStackView) -> Self {
        let ConflictMultiStackView { abilities, marker } = view.clone();
        Self { abilities, marker }
    }
}

impl ddd::Model for ConflictMultiStackModel {
    type View = ConflictMultiStackView;
}
