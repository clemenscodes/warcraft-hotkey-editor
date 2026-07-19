use super::view::VitalityStatsHostView;
use dioxus::prelude::*;
use warcraft_api::{HitPoints, HitPointsRegen, Mana, ManaRegen};

#[derive(Props, Clone, PartialEq)]
pub struct VitalityStatsHostModel {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl From<&VitalityStatsHostView> for VitalityStatsHostModel {
    fn from(view: &VitalityStatsHostView) -> Self {
        let VitalityStatsHostView {
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
        } = view.clone();
        Self {
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
        }
    }
}

impl ddd::Model for VitalityStatsHostModel {
    type View = VitalityStatsHostView;
}
