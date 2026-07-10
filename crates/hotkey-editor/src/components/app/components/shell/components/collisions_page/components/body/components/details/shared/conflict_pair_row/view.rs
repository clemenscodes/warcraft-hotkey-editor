use super::state::AbilityPair;

/// The published `View` contract mirroring [`ConflictPairRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictPairRowView {
    pub pair: Option<AbilityPair>,
}

impl ddd::View for ConflictPairRowView {}
