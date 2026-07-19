use super::view::DefenseStatsHostView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_api::{ArmorFigure as Armor, EffectiveHitPoints, Evasion};

#[derive(Props, Clone, PartialEq)]
pub struct DefenseStatsHostModel {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}

impl From<&DefenseStatsHostView> for DefenseStatsHostModel {
    fn from(view: &DefenseStatsHostView) -> Self {
        let DefenseStatsHostView {
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

impl ddd::Model for DefenseStatsHostModel {
    type View = DefenseStatsHostView;
}
