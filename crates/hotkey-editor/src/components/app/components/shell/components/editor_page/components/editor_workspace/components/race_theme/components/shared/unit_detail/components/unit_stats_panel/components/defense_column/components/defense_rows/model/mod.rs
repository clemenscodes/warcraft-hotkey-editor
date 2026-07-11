use super::view::DefenseRowsView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;
use warcraft_keybinds::{Armor, EffectiveHitPoints, Evasion};

/// The defense rows' input: the four defense figures the rows present (the icon stays
/// with the column). `defense_type` feeds both the defense-type row and the matchup
/// grid.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseRowsModel {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}

impl From<&DefenseRowsView> for DefenseRowsModel {
    fn from(view: &DefenseRowsView) -> Self {
        let DefenseRowsView {
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

impl ddd::Model for DefenseRowsModel {
    type View = DefenseRowsView;
}
