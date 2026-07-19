use super::view::CombatStatsView;
use dioxus::prelude::*;
use warcraft_api::AttackStatistics;

#[derive(Props, Clone, PartialEq)]
pub struct CombatStatsModel {
    pub attack: Option<AttackStatistics>,
}

impl From<&CombatStatsView> for CombatStatsModel {
    fn from(view: &CombatStatsView) -> Self {
        let CombatStatsView { attack } = view.clone();
        Self { attack }
    }
}

impl ddd::Model for CombatStatsModel {
    type View = CombatStatsView;
}
