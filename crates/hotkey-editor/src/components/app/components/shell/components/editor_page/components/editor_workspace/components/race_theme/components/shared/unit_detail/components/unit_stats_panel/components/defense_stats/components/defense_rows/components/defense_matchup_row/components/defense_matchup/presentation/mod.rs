use super::model::DefenseMatchupModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::matchup::{MatchupStrength, MatchupSubject};
use warcraft_api::Matchup;
use warcraft_api::MatchupPairing;

pub(super) struct DefenseMatchupPresentation {
    pub(super) subject: MatchupSubject,
    pub(super) multiplier: f32,
    pub(super) title: String,
    pub(super) strength: MatchupStrength,
}

pub(super) fn use_defense_matchup(props: &DefenseMatchupModel) -> DefenseMatchupPresentation {
    let attack_type = props.attack_type;
    let defense_type = props.defense_type;
    let matchup = Matchup::from(MatchupPairing::new(attack_type, defense_type));
    let multiplier = matchup.multiplier();
    let subject = MatchupSubject::Attack(attack_type);
    let title = format!("{attack_type} attacks");
    let attacker_strength = matchup.strength();
    let defender_strength = attacker_strength.inverted();
    let strength = MatchupStrength::from(defender_strength);
    DefenseMatchupPresentation {
        subject,
        multiplier,
        title,
        strength,
    }
}

impl ddd::Presentation for DefenseMatchupPresentation {
    type Model = DefenseMatchupModel;
}
