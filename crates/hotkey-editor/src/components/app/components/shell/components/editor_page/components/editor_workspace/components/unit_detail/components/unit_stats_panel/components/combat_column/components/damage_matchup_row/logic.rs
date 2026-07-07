use super::components::attack_matchup::AttackMatchupProps;
use super::props::DamageMatchupRowProps;
use warcraft_api::DefenseType;

const DISPLAYED_DEFENSE_TYPES: [DefenseType; 7] = [
    DefenseType::Light,
    DefenseType::Medium,
    DefenseType::Heavy,
    DefenseType::Fortified,
    DefenseType::Hero,
    DefenseType::Divine,
    DefenseType::Unarmored,
];

/// One matchup cell per displayed defense type against this attack.
pub(super) fn cells(props: &DamageMatchupRowProps) -> Vec<AttackMatchupProps> {
    let attack_type = props.attack_type;
    DISPLAYED_DEFENSE_TYPES
        .into_iter()
        .map(|defense_type| AttackMatchupProps {
            defense_type,
            attack_type,
        })
        .collect()
}
