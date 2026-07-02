use crate::components::unit_detail::components::unit_stats_panel::components::matchup_cell::{
    MatchupCellProps, MatchupStrength,
};
use dioxus::prelude::*;
use num_traits::cast::cast;
use warcraft_api::{AttackType, DefenseType};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

/// One cell of a defender's matchup: how an attack type fares against the defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupCellProps {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}

impl From<&DefenseMatchupCellProps> for MatchupCellProps {
    fn from(props: &DefenseMatchupCellProps) -> Self {
        let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(props.attack_type);
        let multiplier = effectiveness.against(props.defense_type);
        let percent_int: i32 = cast::<f32, i32>((multiplier * 100.0).round()).unwrap_or(0);
        let value = format!("{percent_int}%");
        let label = props.attack_type.to_string();
        let title = format!("{label} attacks");
        let strength = if multiplier > 1.05 {
            MatchupStrength::Weak
        } else if multiplier < 0.95 {
            MatchupStrength::Strong
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
