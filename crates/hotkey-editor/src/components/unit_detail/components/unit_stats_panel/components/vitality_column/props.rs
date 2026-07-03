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
