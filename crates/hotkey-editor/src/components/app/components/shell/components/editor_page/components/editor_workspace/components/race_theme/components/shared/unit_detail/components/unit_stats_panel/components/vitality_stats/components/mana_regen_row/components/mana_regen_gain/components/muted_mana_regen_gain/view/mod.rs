/// The published `View` contract mirroring [`MutedManaRegenGainModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MutedManaRegenGainView {
    pub text: String,
}

impl ddd::View for MutedManaRegenGainView {}
