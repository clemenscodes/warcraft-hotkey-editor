use warcraft_api::AttackStatistics;

/// The published `View` contract mirroring [`CombatStatsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CombatStatsView {
    pub attack: Option<AttackStatistics>,
}

impl ddd::View for CombatStatsView {}
