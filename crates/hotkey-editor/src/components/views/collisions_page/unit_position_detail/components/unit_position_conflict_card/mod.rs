mod logic;
mod props;

use crate::components::views::collisions_page::conflict_ability::ConflictAbility;
use crate::components::views::collisions_page::conflict_ability_row::ConflictAbilityRow;
use crate::components::views::collisions_page::conflict_card::ConflictCard;
use crate::components::views::collisions_page::conflict_card_caption::ConflictCardCaption;
use crate::components::views::collisions_page::conflict_position_cell::{
    ConflictPositionCell, ConflictPositionCellProps,
};
use dioxus::prelude::*;
use logic::UnitPositionConflictCardModel;
pub use props::UnitPositionConflictCardProps;

/// One position-collision card: the abilities that land on the same command-card
/// cell, flanking (or stacked under) a mini grid flagging that cell.
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = UnitPositionConflictCardModel::from(&props);
    let mut abilities = model.abilities.into_iter();
    let cell_between = ConflictPositionCellProps {
        collision_column: model.position_column,
        collision_row: model.position_row,
        is_top: false,
    };
    let cell_top = ConflictPositionCellProps {
        collision_column: model.position_column,
        collision_row: model.position_row,
        is_top: true,
    };
    let pair_left = abilities.next();
    let pair_right = abilities.next();
    rsx! {
        ConflictCard {
            ConflictCardCaption { text: model.role_label }
            if model.is_pair {
                ConflictAbilityRow {
                    if let Some(left) = pair_left {
                        ConflictAbility { ..left }
                    }
                    ConflictPositionCell { ..cell_between }
                    if let Some(right) = pair_right {
                        ConflictAbility { ..right }
                    }
                }
            } else {
                ConflictPositionCell { ..cell_top }
                ConflictAbilityRow {
                    is_multi: true,
                    if let Some(left) = pair_left {
                        ConflictAbility { ..left }
                    }
                    if let Some(right) = pair_right {
                        ConflictAbility { ..right }
                    }
                    for ability in abilities {
                        ConflictAbility { ..ability }
                    }
                }
            }
        }
    }
}
