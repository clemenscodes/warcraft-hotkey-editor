use super::view::VitalityStatsView;
use dioxus::prelude::*;
use warcraft_api::{HitPoints, HitPointsRegen, Mana, ManaRegen};

/// The vitality column's input: the unit's resolved hit points, mana, and their
/// regeneration figures.
#[derive(Props, Clone, PartialEq)]
pub struct VitalityStatsModel {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl From<&VitalityStatsView> for VitalityStatsModel {
    fn from(view: &VitalityStatsView) -> Self {
        let VitalityStatsView {
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

impl ddd::Model for VitalityStatsModel {
    type View = VitalityStatsView;
}
