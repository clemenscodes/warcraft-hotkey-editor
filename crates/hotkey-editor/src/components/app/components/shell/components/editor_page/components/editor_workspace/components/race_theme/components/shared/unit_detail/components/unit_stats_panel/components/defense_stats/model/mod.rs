use super::view::DefenseStatsView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_api::{ArmorFigure as Armor, EffectiveHitPoints, Evasion};

/// The defense column's input: the unit's resolved armor, defense type, effective hit
/// points, and dodge chance.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseStatsModel {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}

impl From<&DefenseStatsView> for DefenseStatsModel {
    fn from(view: &DefenseStatsView) -> Self {
        let DefenseStatsView {
            armor,
            defense_type,
            effective_hit_points,
            evasion,
        } = view.clone();
        Self {
            armor,
            defense_type,
            effective_hit_points,
            evasion,
        }
    }
}

impl ddd::Model for DefenseStatsModel {
    type View = DefenseStatsView;
}
