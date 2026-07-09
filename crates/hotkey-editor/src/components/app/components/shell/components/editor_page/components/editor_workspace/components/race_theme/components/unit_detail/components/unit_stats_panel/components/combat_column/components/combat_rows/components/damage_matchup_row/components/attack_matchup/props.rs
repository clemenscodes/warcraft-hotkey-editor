use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::matchup::{
    MatchupProps, MatchupStrength, MatchupSubject,
};
use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};
use warcraft_keybinds::Matchup;

/// One cell of an attacker's damage matchup: how the attack fares against a defense.
#[derive(Props, Clone, PartialEq)]
pub struct AttackMatchupProps {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}

impl From<&AttackMatchupProps> for MatchupProps {
    fn from(props: &AttackMatchupProps) -> Self {
        let attack_type = props.attack_type;
        let defense_type = props.defense_type;
        let matchup = Matchup::resolve(attack_type, defense_type);
        let multiplier = matchup.multiplier();
        let subject = MatchupSubject::Defense(defense_type);
        let title = format!("vs {defense_type}");
        let attacker_strength = matchup.strength();
        let strength = MatchupStrength::from(attacker_strength);
        Self {
            subject,
            multiplier,
            title,
            strength,
        }
    }
}
