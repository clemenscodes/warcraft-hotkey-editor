use warcraft_api::HitPointsRegen;

#[derive(Clone, PartialEq)]
pub struct HitPointsRegenRowView {
    pub value: HitPointsRegen,
}

impl ddd::View for HitPointsRegenRowView {}
