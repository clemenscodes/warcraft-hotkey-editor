mod logic;
mod props;

use super::resolve_ability_icon::ResolveAbilityIcon;
use super::resolve_fight_col::ResolveFightCol;
use super::resolve_fight_name_btn::ResolveFightNameBtn;
use super::resolve_fight_name_plate::ResolveFightNamePlate;
use super::resolve_fight_row::ResolveFightRow;
use super::resolve_grid_col::ResolveGridCol;
use super::resolve_mini_grid::ResolveMiniGrid;
use super::resolve_move_arrow::ResolveMoveArrow;
use super::resolve_move_card::ResolveMoveCard;
use super::resolve_move_reason_row::ResolveMoveReasonRow;
use super::resolve_move_transition::ResolveMoveTransition;
use dioxus::prelude::*;
use logic::{AnchorParts, ResolveMoveRowModel};
pub use props::ResolveMoveRowProps;

/// One move card: the reason badge, the fighting abilities (names over icons), and
/// the from → to grids drawing where each ability lands.
#[component]
pub fn ResolveMoveRow(props: ResolveMoveRowProps) -> Element {
    let model = ResolveMoveRowModel::from(&props);
    rsx! {
        ResolveMoveCard {
            ResolveMoveReasonRow { ..model.reason_row }
            ResolveFightRow {
                ResolveFightCol {
                    ResolveFightNameBtn { ..model.mover_name_btn }
                    ResolveAbilityIcon { ..model.mover_ability }
                }
                if let Some(AnchorParts { name_plate, ability }) = model.anchor {
                    ResolveFightCol {
                        ResolveFightNamePlate { ..name_plate }
                        ResolveAbilityIcon { ..ability }
                    }
                }
            }
            ResolveMoveTransition {
                ResolveGridCol {
                    ResolveMiniGrid { placements: model.from_placements }
                }
                ResolveMoveArrow {}
                ResolveGridCol {
                    ResolveMiniGrid { placements: model.to_placements }
                }
            }
        }
    }
}
