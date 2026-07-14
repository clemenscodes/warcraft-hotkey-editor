use warcraft_api::HitPointsRegen;

#[derive(Clone, PartialEq)]
pub struct HitPointsRegenGainView {
    pub value: HitPointsRegen,
}

impl ddd::View for HitPointsRegenGainView {}
