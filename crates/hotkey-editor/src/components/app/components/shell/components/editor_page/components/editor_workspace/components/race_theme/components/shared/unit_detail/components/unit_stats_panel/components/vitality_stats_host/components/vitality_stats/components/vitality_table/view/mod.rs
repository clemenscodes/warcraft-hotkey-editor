use warcraft_api::{HitPoints, HitPointsRegen, Mana, ManaRegen};

#[derive(Clone, PartialEq)]
pub struct VitalityTableView {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl ddd::Layered for VitalityTableView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for VitalityTableView {}
