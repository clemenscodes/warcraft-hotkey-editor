use super::props::DefenseMatchupProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::{MatchupStrength, MatchupSubject};
use warcraft_keybinds::Matchup;

/// The shaped matchup figures for one defender cell.
pub(super) struct DefenseMatchupModel {
    pub(super) subject: MatchupSubject,
    pub(super) multiplier: f32,
    pub(super) title: String,
    pub(super) strength: MatchupStrength,
}

pub(super) fn use_defense_matchup(props: &DefenseMatchupProps) -> DefenseMatchupModel {
    let attack_type = props.attack_type;
    let defense_type = props.defense_type;
    let matchup = Matchup::resolve(attack_type, defense_type);
    let multiplier = matchup.multiplier();
    let subject = MatchupSubject::Attack(attack_type);
    let title = format!("{attack_type} attacks");
    let attacker_strength = matchup.strength();
    let defender_strength = attacker_strength.inverted();
    let strength = MatchupStrength::from(defender_strength);
    DefenseMatchupModel {
        subject,
        multiplier,
        title,
        strength,
    }
}
