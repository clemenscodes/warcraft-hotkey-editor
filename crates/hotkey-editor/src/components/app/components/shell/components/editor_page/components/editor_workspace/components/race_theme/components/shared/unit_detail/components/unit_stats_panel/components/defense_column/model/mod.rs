use super::view::DefenseColumnView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_api::{ArmorFigure as Armor, EffectiveHitPoints, Evasion};

/// The defense column's input: the unit's resolved armor, defense type, effective hit
/// points, and dodge chance.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseColumnModel {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}

impl From<&DefenseColumnView> for DefenseColumnModel {
    fn from(view: &DefenseColumnView) -> Self {
        let DefenseColumnView {
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

impl ddd::Model for DefenseColumnModel {
    type View = DefenseColumnView;
}
