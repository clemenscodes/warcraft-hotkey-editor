use warcraft_api::EffectiveHitPoints;

#[derive(Clone, PartialEq)]
pub struct EffectiveHitPointsRowView {
    pub value: EffectiveHitPoints,
}

impl ddd::Layered for EffectiveHitPointsRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EffectiveHitPointsRowView {}
