use super::components::defense_matchup_cell::DefenseMatchupCellProps;
use super::props::DefenseMatchupRowProps;
use warcraft_api::AttackType;

const ALL_ATTACK_TYPES: [AttackType; 7] = [
    AttackType::Normal,
    AttackType::Pierce,
    AttackType::Siege,
    AttackType::Magic,
    AttackType::Chaos,
    AttackType::Hero,
    AttackType::Spells,
];

/// One matchup cell per attack type against this defense.
pub(super) fn cells(props: &DefenseMatchupRowProps) -> Vec<DefenseMatchupCellProps> {
    let defense_type = props.defense_type;
    ALL_ATTACK_TYPES
        .into_iter()
        .map(|attack_type| DefenseMatchupCellProps {
            attack_type,
            defense_type,
        })
        .collect()
}
