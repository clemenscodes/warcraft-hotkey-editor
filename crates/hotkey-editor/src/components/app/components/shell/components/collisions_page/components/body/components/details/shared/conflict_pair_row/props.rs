use super::super::conflict_ability::ConflictAbilityProps;
use super::super::conflict_marker_view::ConflictMarker;
use dioxus::prelude::*;

/// The two abilities flanking the conflict marker in a pair clash, with the marker.
#[derive(Clone, PartialEq)]
pub struct AbilityPair {
    pub(super) left: ConflictAbilityProps,
    pub(super) right: ConflictAbilityProps,
    pub(super) marker: ConflictMarker,
}

impl AbilityPair {
    pub fn new(
        left: ConflictAbilityProps,
        right: ConflictAbilityProps,
        marker: ConflictMarker,
    ) -> Self {
        Self {
            left,
            right,
            marker,
        }
    }
}

/// The pair-clash row: two abilities flanking the marker, or nothing when the clash
/// is not an exact pair.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPairRowProps {
    pub pair: Option<AbilityPair>,
}
