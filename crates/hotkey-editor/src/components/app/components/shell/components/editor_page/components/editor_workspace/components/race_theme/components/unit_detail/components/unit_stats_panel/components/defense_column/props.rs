use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_keybinds::{Armor, EffectiveHitPoints, Evasion};

/// The defense column's input: the unit's resolved armor, defense type, effective hit
/// points, and dodge chance.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseColumnProps {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}
