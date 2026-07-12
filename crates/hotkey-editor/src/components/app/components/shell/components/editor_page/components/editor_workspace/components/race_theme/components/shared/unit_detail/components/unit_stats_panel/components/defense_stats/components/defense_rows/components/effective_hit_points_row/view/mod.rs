use warcraft_api::EffectiveHitPoints;

/// The published `View` contract mirroring [`EffectiveHitPointsRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EffectiveHitPointsRowView {
    pub value: EffectiveHitPoints,
}

impl ddd::View for EffectiveHitPointsRowView {}
