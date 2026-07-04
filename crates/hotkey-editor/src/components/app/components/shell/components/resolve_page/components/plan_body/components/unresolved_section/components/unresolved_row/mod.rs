mod data;
mod logic;
mod props;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_column::FightColumn;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlate;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_row::FightRow;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::mini_grid::MiniGrid;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_card::MoveCard;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_transition::MoveTransition;
use dioxus::prelude::*;
use logic::UnresolvedRowModel;
pub use props::UnresolvedRowProps;

/// One unresolved ability card: the Stuck badge, the ability, and the cell it is
/// stuck on.
#[component]
pub fn UnresolvedRow(props: UnresolvedRowProps) -> Element {
    let model = UnresolvedRowModel::from(&props);
    rsx! {
        MoveCard {
            is_stuck: true,
            MoveReasonRow { ..model.reason_row }
            FightRow {
                FightColumn {
                    FightNamePlate { ..model.name_plate }
                    AbilityIcon { ..model.ability }
                }
            }
            MoveTransition {
                MiniGrid { placements: model.placements }
            }
        }
    }
}
