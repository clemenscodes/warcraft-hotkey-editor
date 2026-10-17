use warcraft_api::AttackStatistics;

#[derive(Clone, PartialEq)]
pub struct CombatStatsHostView {
    pub attack: Option<AttackStatistics>,
}

impl ddd::Layered for CombatStatsHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CombatStatsHostView {}
