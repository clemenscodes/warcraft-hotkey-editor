use warcraft_api::EffectiveHitPoints;

#[derive(Clone, PartialEq)]
pub struct EffectiveHitPointsRowView {
    pub value: EffectiveHitPoints,
}

impl ddd::View for EffectiveHitPointsRowView {}
