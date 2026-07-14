/// The published `View` contract mirroring [`AbilityNameModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityNameView {
    pub text: String,
}

impl ddd::View for AbilityNameView {}
