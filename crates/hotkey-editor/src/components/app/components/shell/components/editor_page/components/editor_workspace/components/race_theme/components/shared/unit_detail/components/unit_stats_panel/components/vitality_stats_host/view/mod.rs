use warcraft_api::{HitPoints, HitPointsRegen, Mana, ManaRegen};

#[derive(Clone, PartialEq)]
pub struct VitalityStatsHostView {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl ddd::View for VitalityStatsHostView {}
