use super::view::VitalityColumnView;
use dioxus::prelude::*;
use warcraft_keybinds::{HitPoints, HitPointsRegen, Mana, ManaRegen};

/// The vitality column's input: the unit's resolved hit points, mana, and their
/// regeneration figures.
#[derive(Props, Clone, PartialEq)]
pub struct VitalityColumnProps {
    pub hit_points: HitPoints,
    pub hit_points_regen: HitPointsRegen,
    pub mana: Mana,
    pub mana_regen: ManaRegen,
}

impl From<&VitalityColumnView> for VitalityColumnProps {
    fn from(view: &VitalityColumnView) -> Self {
        let VitalityColumnView {
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

impl ddd::Props for VitalityColumnProps {
    type View = VitalityColumnView;
}
