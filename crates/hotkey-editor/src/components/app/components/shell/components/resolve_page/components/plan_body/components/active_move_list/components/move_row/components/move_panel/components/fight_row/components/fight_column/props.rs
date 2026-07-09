use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use super::components::fight_name_button::FightNameButtonProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::active_move_list::components::move_row::components::move_panel::components::fight_row::FightRowProps;
use dioxus::prelude::*;

/// The mover's fighter column: its name button stacked over its ability icon.
#[derive(Props, Clone, PartialEq)]
pub struct FightColumnProps {
    pub mover_name_btn: FightNameButtonProps,
    pub mover_ability: AbilityIconProps,
}

impl From<&FightRowProps> for FightColumnProps {
    fn from(props: &FightRowProps) -> Self {
        let mover_name_btn = props.mover_name_btn.clone();
        let mover_ability = props.mover_ability.clone();
        Self {
            mover_name_btn,
            mover_ability,
        }
    }
}
