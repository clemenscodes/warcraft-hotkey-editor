use super::state::AbilityPair;
use dioxus::prelude::*;

/// The pair-clash row: two abilities flanking the marker, or nothing when the clash
/// is not an exact pair.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPairRowProps {
    pub pair: Option<AbilityPair>,
}
