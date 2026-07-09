use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::matchup::{
    MatchupProps, MatchupStrength, MatchupSubject,
};
use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};
use warcraft_keybinds::Matchup;

/// One cell of a defender's matchup: how an attack type fares against the defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupProps {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}

impl From<&DefenseMatchupProps> for MatchupProps {
    fn from(props: &DefenseMatchupProps) -> Self {
        let attack_type = props.attack_type;
        let defense_type = props.defense_type;
        let matchup = Matchup::resolve(attack_type, defense_type);
        let multiplier = matchup.multiplier();
        let subject = MatchupSubject::Attack(attack_type);
        let title = format!("{attack_type} attacks");
        let attacker_strength = matchup.strength();
        let defender_strength = attacker_strength.inverted();
        let strength = MatchupStrength::from(defender_strength);
        Self {
            subject,
            multiplier,
            title,
            strength,
        }
    }
}
