/// The published `View` contract mirroring [`ActiveStatGainProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveStatGainView {
    pub text: String,
}

impl ddd::View for ActiveStatGainView {}
