/// The published `View` contract mirroring [`ActiveHitPointsRegenGainModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainView {
    pub text: String,
}

impl ddd::View for ActiveHitPointsRegenGainView {}
