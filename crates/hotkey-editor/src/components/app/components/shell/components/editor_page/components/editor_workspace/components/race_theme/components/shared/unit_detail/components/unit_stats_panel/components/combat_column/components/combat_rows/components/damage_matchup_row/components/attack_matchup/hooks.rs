use super::props::AttackMatchupProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::{MatchupStrength, MatchupSubject};
use warcraft_keybinds::Matchup;

/// The shaped matchup figures for one attacker cell.
pub(super) struct AttackMatchupModel {
    pub(super) subject: MatchupSubject,
    pub(super) multiplier: f32,
    pub(super) title: String,
    pub(super) strength: MatchupStrength,
}

pub(super) fn use_attack_matchup(props: &AttackMatchupProps) -> AttackMatchupModel {
    let attack_type = props.attack_type;
    let defense_type = props.defense_type;
    let matchup = Matchup::resolve(attack_type, defense_type);
    let multiplier = matchup.multiplier();
    let subject = MatchupSubject::Defense(defense_type);
    let title = format!("vs {defense_type}");
    let attacker_strength = matchup.strength();
    let strength = MatchupStrength::from(attacker_strength);
    AttackMatchupModel {
        subject,
        multiplier,
        title,
        strength,
    }
}
