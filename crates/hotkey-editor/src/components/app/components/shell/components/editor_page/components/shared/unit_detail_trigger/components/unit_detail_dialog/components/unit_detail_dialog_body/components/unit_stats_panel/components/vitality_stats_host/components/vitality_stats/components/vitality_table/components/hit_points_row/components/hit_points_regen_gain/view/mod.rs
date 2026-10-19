use warcraft_api::HitPointsRegen;

#[derive(Clone, PartialEq)]
pub struct HitPointsRegenGainView {
    pub value: HitPointsRegen,
}

impl ddd::Layered for HitPointsRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HitPointsRegenGainView {}
