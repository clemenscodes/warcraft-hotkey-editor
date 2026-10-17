use warcraft_api::AttackStatistics;

#[derive(Clone, PartialEq)]
pub struct CombatStatsView {
    pub attack: Option<AttackStatistics>,
}

impl ddd::Layered for CombatStatsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CombatStatsView {}
