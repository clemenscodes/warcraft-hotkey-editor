use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_stats_panel::components::shared::matchup::{
    MatchupProps, MatchupStrength,
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
        let percent = matchup.multiplier() * 100.0;
        let value = format!("{percent:.0}%");
        let label = defense_type.to_string();
        let title = format!("vs {label}");
        let attacker_strength = matchup.strength();
        let strength = MatchupStrength::from(attacker_strength);
        Self {
            label,
            value,
            title,
            strength,
        }
    }
}
