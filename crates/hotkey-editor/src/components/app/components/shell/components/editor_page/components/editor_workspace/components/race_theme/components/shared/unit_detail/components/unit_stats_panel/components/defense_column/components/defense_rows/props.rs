use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_keybinds::{Armor, EffectiveHitPoints, Evasion};

/// The defense rows' input: the four defense figures the rows present (the icon stays
/// with the column). `defense_type` feeds both the defense-type row and the matchup
/// grid.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseRowsProps {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}
