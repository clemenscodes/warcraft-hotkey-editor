use warcraft_api::DefenseType;
use warcraft_keybinds::{Armor, EffectiveHitPoints, Evasion};

/// The published `View` contract mirroring [`DefenseRowsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DefenseRowsView {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}

impl ddd::View for DefenseRowsView {}
