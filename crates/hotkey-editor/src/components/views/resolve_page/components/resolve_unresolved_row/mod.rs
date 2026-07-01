mod logic;
mod props;

use super::resolve_ability_icon::ResolveAbilityIcon;
use super::resolve_fight_col::ResolveFightCol;
use super::resolve_fight_name_plate::ResolveFightNamePlate;
use super::resolve_fight_row::ResolveFightRow;
use super::resolve_mini_grid::ResolveMiniGrid;
use super::resolve_move_card::ResolveMoveCard;
use super::resolve_move_reason_row::ResolveMoveReasonRow;
use super::resolve_move_transition::ResolveMoveTransition;
use crate::components::views::resolve_page::logic::ResolveReasonKind;
use dioxus::prelude::*;
use logic::ResolveUnresolvedRowModel;
pub use props::ResolveUnresolvedRowProps;

/// One unresolved ability card: the Stuck badge, the ability, and the cell it is
/// stuck on.
#[component]
pub fn ResolveUnresolvedRow(props: ResolveUnresolvedRowProps) -> Element {
    let model = ResolveUnresolvedRowModel::from(&props);
    rsx! {
        ResolveMoveCard {
            is_stuck: true,
            ResolveMoveReasonRow { kind: ResolveReasonKind::Stuck, label: "Stuck" }
            ResolveFightRow {
                ResolveFightCol {
                    ResolveFightNamePlate { ..model.name_plate }
                    ResolveAbilityIcon { ..model.ability }
                }
            }
            ResolveMoveTransition {
                ResolveMiniGrid { placements: model.placements }
            }
        }
    }
}
