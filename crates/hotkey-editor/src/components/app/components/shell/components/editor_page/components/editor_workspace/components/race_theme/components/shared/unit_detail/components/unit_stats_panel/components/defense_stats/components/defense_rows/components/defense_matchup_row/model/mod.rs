use super::view::DefenseMatchupRowView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;

/// The defender's matchup grid: how each attack type fares against this defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupRowModel {
    pub defense_type: DefenseType,
}

impl From<&DefenseMatchupRowView> for DefenseMatchupRowModel {
    fn from(view: &DefenseMatchupRowView) -> Self {
        let DefenseMatchupRowView { defense_type } = view.clone();
        Self { defense_type }
    }
}

impl ddd::Model for DefenseMatchupRowModel {
    type View = DefenseMatchupRowView;
}
