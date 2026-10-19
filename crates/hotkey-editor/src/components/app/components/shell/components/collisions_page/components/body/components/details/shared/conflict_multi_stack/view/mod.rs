use super::super::conflict_card_model::ConflictAbilityData;
use super::super::conflict_marker::ConflictMarker;

#[derive(Clone, PartialEq)]
pub struct ConflictMultiStackView {
    pub(crate) abilities: Vec<ConflictAbilityData>,
    pub marker: ConflictMarker,
}

impl ddd::Layered for ConflictMultiStackView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictMultiStackView {}
