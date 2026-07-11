/// The published `View` contract mirroring [`MutedHitPointsRegenGainModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MutedHitPointsRegenGainView {
    pub text: String,
}

impl ddd::View for MutedHitPointsRegenGainView {}
