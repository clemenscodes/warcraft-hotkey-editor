/// The published `View` contract mirroring [`ConflictAbilityNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictAbilityNameView {
    pub text: String,
}

impl ddd::View for ConflictAbilityNameView {}
