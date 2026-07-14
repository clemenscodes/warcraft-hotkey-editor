use super::state::AbilityPair;

#[derive(Clone, PartialEq)]
pub struct ConflictPairRowView {
    pub pair: Option<AbilityPair>,
}

impl ddd::View for ConflictPairRowView {}
