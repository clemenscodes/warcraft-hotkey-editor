use warcraft_keybinds::EffectiveHitPoints;

/// The published `View` contract mirroring [`EffectiveHitPointsRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EffectiveHitPointsRowView {
    pub value: EffectiveHitPoints,
}

impl ddd::View for EffectiveHitPointsRowView {}
