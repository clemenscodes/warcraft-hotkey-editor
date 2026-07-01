use super::super::matchup_cell::{MatchupCellProps, MatchupStrength};
use dioxus::prelude::*;
use num_traits::cast::cast;
use warcraft_api::{AttackType, DefenseType};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

/// One cell of an attacker's damage matchup: how the attack fares against a defense.
#[derive(Props, Clone, PartialEq)]
pub struct AttackMatchupCellProps {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}

impl From<&AttackMatchupCellProps> for MatchupCellProps {
    fn from(props: &AttackMatchupCellProps) -> Self {
        let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(props.attack_type);
        let multiplier = effectiveness.against(props.defense_type);
        let percent_int: i32 = cast::<f32, i32>((multiplier * 100.0).round()).unwrap_or(0);
        let value = format!("{percent_int}%");
        let label = props.defense_type.to_string();
        let title = format!("vs {label}");
        let strength = if multiplier > 1.05 {
            MatchupStrength::Strong
        } else if multiplier < 0.95 {
            MatchupStrength::Weak
        } else {
            MatchupStrength::Neutral
        };
        Self {
            label,
            value,
            title,
            strength,
        }
    }
}
