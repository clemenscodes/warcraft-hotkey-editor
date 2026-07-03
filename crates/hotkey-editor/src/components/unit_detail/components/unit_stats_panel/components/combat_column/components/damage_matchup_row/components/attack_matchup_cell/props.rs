use crate::components::unit_detail::components::unit_stats_panel::components::shared::matchup_cell::{
    MatchupCellProps, MatchupStrength,
};
use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};
use warcraft_keybinds::Matchup;

/// One cell of an attacker's damage matchup: how the attack fares against a defense.
#[derive(Props, Clone, PartialEq)]
pub struct AttackMatchupCellProps {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}

impl From<&AttackMatchupCellProps> for MatchupCellProps {
    fn from(props: &AttackMatchupCellProps) -> Self {
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
