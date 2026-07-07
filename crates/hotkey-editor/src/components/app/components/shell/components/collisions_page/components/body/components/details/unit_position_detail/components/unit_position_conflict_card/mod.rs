pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card::ConflictCard;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use components::position_multi_stack::PositionMultiStack;
use components::position_pair_row::PositionPairRow;
use dioxus::prelude::*;
use logic::UnitPositionConflictCardModel;
pub use props::UnitPositionConflictCardProps;

/// One position-collision card: the abilities that land on the same command-card
/// cell, flanking (or stacked under) a mini grid flagging that cell. Each of the two
/// layouts renders itself away when it does not apply.
use tw_macro::assert_component;
assert_component!(UnitPositionConflictCard);
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = UnitPositionConflictCardModel::from(&props);
    rsx! {
        ConflictCard {
            ConflictCardCaption { text: model.role_label }
            PositionPairRow { ..model.pair_row }
            PositionMultiStack { ..model.multi_stack }
        }
    }
}
