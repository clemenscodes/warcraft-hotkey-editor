/// The published `View` contract mirroring [`MutedStatValueModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MutedStatValueView {
    pub text: String,
}

impl ddd::View for MutedStatValueView {}
