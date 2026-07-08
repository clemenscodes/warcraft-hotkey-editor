use dioxus::prelude::*;
use warcraft_api::DefenseType;

/// The defender's matchup grid: how each attack type fares against this defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupRowProps {
    pub defense_type: DefenseType,
}
