/// The published `View` contract mirroring [`ConflictUnitNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictUnitNameView {
    pub text: String,
}

impl ddd::View for ConflictUnitNameView {}
