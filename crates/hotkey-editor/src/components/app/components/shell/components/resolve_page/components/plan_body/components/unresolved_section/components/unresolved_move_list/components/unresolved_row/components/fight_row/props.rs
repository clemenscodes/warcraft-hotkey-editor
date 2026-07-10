use super::components::fight_column::FightColumnProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlateProps;
use dioxus::prelude::*;

/// The stuck ability's row: the single centered column holding the stuck ability.
#[derive(Props, Clone, PartialEq)]
pub struct FightRowProps {
    pub name_plate: FightNamePlateProps,
    pub ability: AbilityIconProps,
}

impl From<&FightRowProps> for FightColumnProps {
    fn from(props: &FightRowProps) -> Self {
        let name_plate = props.name_plate.clone();
        let ability = props.ability.clone();
        Self {
            name_plate,
            ability,
        }
    }
}
