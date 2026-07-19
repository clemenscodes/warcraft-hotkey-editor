use super::view::CombatStatsHostView;
use dioxus::prelude::*;
use warcraft_api::AttackStatistics;

#[derive(Props, Clone, PartialEq)]
pub struct CombatStatsHostModel {
    pub attack: Option<AttackStatistics>,
}

impl From<&CombatStatsHostView> for CombatStatsHostModel {
    fn from(view: &CombatStatsHostView) -> Self {
        let CombatStatsHostView { attack } = view.clone();
        Self { attack }
    }
}

impl ddd::Model for CombatStatsHostModel {
    type View = CombatStatsHostView;
}
