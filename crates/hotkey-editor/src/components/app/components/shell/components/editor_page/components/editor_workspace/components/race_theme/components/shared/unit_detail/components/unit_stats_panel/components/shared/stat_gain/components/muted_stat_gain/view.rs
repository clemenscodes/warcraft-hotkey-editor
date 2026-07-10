/// The published `View` contract mirroring [`MutedStatGainProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MutedStatGainView {
    pub text: String,
}

impl ddd::View for MutedStatGainView {}
