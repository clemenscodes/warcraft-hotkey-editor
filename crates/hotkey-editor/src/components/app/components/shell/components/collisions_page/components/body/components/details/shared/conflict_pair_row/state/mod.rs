use super::super::conflict_card_model::ConflictAbilityData;
use super::super::conflict_marker_view::ConflictMarker;

/// The two abilities flanking the conflict marker in a pair clash, with the marker.
#[derive(Clone, PartialEq)]
pub struct AbilityPair {
    pub(super) left: ConflictAbilityData,
    pub(super) right: ConflictAbilityData,
    pub(super) marker: ConflictMarker,
}

impl AbilityPair {
    pub(crate) fn new(
        left: ConflictAbilityData,
        right: ConflictAbilityData,
        marker: ConflictMarker,
    ) -> Self {
        Self {
            left,
            right,
            marker,
        }
    }
}
