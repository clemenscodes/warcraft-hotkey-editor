use super::view::DefenseMatchupRowView;
use dioxus::prelude::*;
use warcraft_api::DefenseType;

/// The defender's matchup grid: how each attack type fares against this defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupRowProps {
    pub defense_type: DefenseType,
}

impl From<&DefenseMatchupRowView> for DefenseMatchupRowProps {
    fn from(view: &DefenseMatchupRowView) -> Self {
        let DefenseMatchupRowView { defense_type } = view.clone();
        Self { defense_type }
    }
}

impl ddd::Props for DefenseMatchupRowProps {
    type View = DefenseMatchupRowView;
}
