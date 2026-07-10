/// The published `View` contract mirroring [`MutedManaValueProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MutedManaValueView {
    pub text: String,
}

impl ddd::View for MutedManaValueView {}
