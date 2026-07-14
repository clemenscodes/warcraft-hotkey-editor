#[derive(Clone, PartialEq)]
pub struct ConflictUnitNameView {
    pub text: String,
}

impl ddd::View for ConflictUnitNameView {}
