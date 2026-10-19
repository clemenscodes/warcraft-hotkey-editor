use super::state::AbilityPair;

#[derive(Clone, PartialEq)]
pub struct ConflictPairRowView {
    pub pair: Option<AbilityPair>,
}

impl ddd::Layered for ConflictPairRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictPairRowView {}
