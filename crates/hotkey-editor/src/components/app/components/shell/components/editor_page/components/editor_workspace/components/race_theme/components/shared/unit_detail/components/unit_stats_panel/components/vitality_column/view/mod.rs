use warcraft_keybinds::{HitPoints, HitPointsRegen, Mana, ManaRegen};

/// The published `View` contract mirroring [`VitalityColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct VitalityColumnView {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl ddd::View for VitalityColumnView {}
