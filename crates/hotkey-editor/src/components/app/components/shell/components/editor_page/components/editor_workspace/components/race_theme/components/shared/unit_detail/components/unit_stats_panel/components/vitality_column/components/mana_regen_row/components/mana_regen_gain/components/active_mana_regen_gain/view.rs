/// The published `View` contract mirroring [`ActiveManaRegenGainProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveManaRegenGainView {
    pub text: String,
}

impl ddd::View for ActiveManaRegenGainView {}
