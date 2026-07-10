use super::view::DefenseColumnView;
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

impl From<&DefenseColumnView> for DefenseColumnProps {
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

impl ddd::Props for DefenseColumnProps {
    type View = DefenseColumnView;
}
