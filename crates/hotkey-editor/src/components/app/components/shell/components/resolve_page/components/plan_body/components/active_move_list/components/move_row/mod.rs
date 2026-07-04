pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_column::FightColumn;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_row::FightRow;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::mini_grid::MiniGrid;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_card::MoveCard;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_transition::MoveTransition;
use components::anchor_column::AnchorColumn;
use components::fight_name_button::FightNameButton;
use components::grid_column::GridColumn;
use components::move_arrow::MoveArrow;
use dioxus::prelude::*;
use logic::MoveRowModel;
pub use props::MoveRowProps;

/// One move card: the reason badge, the fighting abilities (names over icons), and
/// the from → to grids drawing where each ability lands. The rival column renders
/// itself away when the move has no anchor.
#[component]
pub fn MoveRow(props: MoveRowProps) -> Element {
    let model = MoveRowModel::from(&props);
    rsx! {
        MoveCard {
            MoveReasonRow { ..model.reason_row }
            FightRow {
                FightColumn {
                    FightNameButton { ..model.mover_name_btn }
                    AbilityIcon { ..model.mover_ability }
                }
                AnchorColumn { ..model.anchor }
            }
            MoveTransition {
                GridColumn {
                    MiniGrid { placements: model.from_placements }
                }
                MoveArrow {}
                GridColumn {
                    MiniGrid { placements: model.to_placements }
                }
            }
        }
    }
}
