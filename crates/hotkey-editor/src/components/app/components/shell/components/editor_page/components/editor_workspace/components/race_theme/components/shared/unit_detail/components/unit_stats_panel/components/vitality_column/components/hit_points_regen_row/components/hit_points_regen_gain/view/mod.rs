use warcraft_api::HitPointsRegen;

/// The published `View` contract mirroring [`HitPointsRegenGainModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HitPointsRegenGainView {
    pub value: HitPointsRegen,
}

impl ddd::View for HitPointsRegenGainView {}
