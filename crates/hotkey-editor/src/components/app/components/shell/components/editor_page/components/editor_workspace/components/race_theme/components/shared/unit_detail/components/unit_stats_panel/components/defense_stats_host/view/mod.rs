use warcraft_api::DefenseType;
use warcraft_api::{ArmorFigure as Armor, EffectiveHitPoints, Evasion};

#[derive(Clone, PartialEq)]
pub struct DefenseStatsHostView {
    pub armor: Armor,
    pub defense_type: DefenseType,
    pub effective_hit_points: EffectiveHitPoints,
    pub evasion: Evasion,
}

impl ddd::Layered for DefenseStatsHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DefenseStatsHostView {}
