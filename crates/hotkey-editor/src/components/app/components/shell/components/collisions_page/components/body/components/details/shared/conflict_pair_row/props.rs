use super::state::AbilityPair;
use super::view::ConflictPairRowView;
use dioxus::prelude::*;

/// The pair-clash row: two abilities flanking the marker, or nothing when the clash
/// is not an exact pair.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPairRowProps {
    pub pair: Option<AbilityPair>,
}

impl From<&ConflictPairRowView> for ConflictPairRowProps {
    fn from(view: &ConflictPairRowView) -> Self {
        let ConflictPairRowView { pair } = view.clone();
        Self { pair }
    }
}

impl ddd::Props for ConflictPairRowProps {
    type View = ConflictPairRowView;
}
