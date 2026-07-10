use super::super::conflict_card_model::ConflictAbilityData;
use super::super::conflict_marker_view::ConflictMarker;

/// The published `View` contract mirroring [`ConflictMultiStackProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictMultiStackView {
    pub(crate) abilities: Vec<ConflictAbilityData>,
    pub marker: ConflictMarker,
}

impl ddd::View for ConflictMultiStackView {}
