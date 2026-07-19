use warcraft_api::AttackStatistics;

#[derive(Clone, PartialEq)]
pub struct CombatStatsHostView {
    pub attack: Option<AttackStatistics>,
}

impl ddd::View for CombatStatsHostView {}
