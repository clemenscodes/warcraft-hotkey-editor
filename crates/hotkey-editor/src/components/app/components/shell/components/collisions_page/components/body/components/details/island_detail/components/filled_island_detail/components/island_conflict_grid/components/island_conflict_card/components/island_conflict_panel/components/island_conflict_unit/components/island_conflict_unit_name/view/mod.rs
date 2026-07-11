/// The published `View` contract mirroring [`IslandConflictUnitNameModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictUnitNameView {
    pub text: String,
}

impl ddd::View for IslandConflictUnitNameView {}
