use warcraft_api::AttackStatistics;

#[derive(Clone, PartialEq)]
pub struct CombatStatsView {
    pub attack: Option<AttackStatistics>,
}

impl ddd::View for CombatStatsView {}
