use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlateProps;
use dioxus::prelude::*;

/// The stuck ability's column: its name plate over its ability icon.
#[derive(Props, Clone, PartialEq)]
pub struct FightColumnProps {
    pub name_plate: FightNamePlateProps,
    pub ability: AbilityIconProps,
}

impl From<&FightColumnProps> for FightNamePlateProps {
    fn from(props: &FightColumnProps) -> Self {
        props.name_plate.clone()
    }
}

impl From<&FightColumnProps> for AbilityIconProps {
    fn from(props: &FightColumnProps) -> Self {
        props.ability.clone()
    }
}
