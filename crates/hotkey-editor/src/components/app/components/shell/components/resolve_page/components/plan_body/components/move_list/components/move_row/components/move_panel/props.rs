use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIconProps;
use super::components::fight_row::components::anchor_column::AnchorColumnProps;
use super::components::fight_row::components::fight_column::components::fight_name_button::FightNameButtonProps;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_list::components::move_row::logic::MoveRowModel;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRowProps;
use crate::components::app::components::shell::components::resolve_page::logic::MiniGridPlacement;
use dioxus::prelude::*;

/// The move card's body, shaped by the move row's hook: the reason badge, the mover
/// and rival columns, and the from → to mini-grid placements.
#[derive(Props, Clone, PartialEq)]
pub struct MovePanelProps {
    pub reason_row: MoveReasonRowProps,
    pub mover_name_btn: FightNameButtonProps,
    pub mover_ability: AbilityIconProps,
    pub anchor: AnchorColumnProps,
    pub from_placements: Vec<MiniGridPlacement>,
    pub to_placements: Vec<MiniGridPlacement>,
}

impl From<MoveRowModel> for MovePanelProps {
    fn from(model: MoveRowModel) -> Self {
        Self {
            reason_row: model.reason_row,
            mover_name_btn: model.mover_name_btn,
            mover_ability: model.mover_ability,
            anchor: model.anchor,
            from_placements: model.from_placements,
            to_placements: model.to_placements,
        }
    }
}
