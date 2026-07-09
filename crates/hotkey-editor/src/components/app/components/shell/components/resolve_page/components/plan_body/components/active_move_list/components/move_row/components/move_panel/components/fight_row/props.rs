use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use super::components::anchor_column::AnchorColumnProps;
use super::components::fight_column::components::fight_name_button::FightNameButtonProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::active_move_list::components::move_row::components::move_panel::MovePanelProps;
use dioxus::prelude::*;

/// The fighting-abilities row: the mover column (name button over icon) beside the
/// optional rival column.
#[derive(Props, Clone, PartialEq)]
pub struct FightRowProps {
    pub mover_name_btn: FightNameButtonProps,
    pub mover_ability: AbilityIconProps,
    pub anchor: AnchorColumnProps,
}

impl From<&MovePanelProps> for FightRowProps {
    fn from(props: &MovePanelProps) -> Self {
        let mover_name_btn = props.mover_name_btn.clone();
        let mover_ability = props.mover_ability.clone();
        let anchor = props.anchor.clone();
        Self {
            mover_name_btn,
            mover_ability,
            anchor,
        }
    }
}
