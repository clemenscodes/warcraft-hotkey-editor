use warcraft_api::HitPointsRegen;

#[derive(Clone, PartialEq)]
pub struct HitPointsRegenRowView {
    pub value: HitPointsRegen,
}

impl ddd::Layered for HitPointsRegenRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HitPointsRegenRowView {}
