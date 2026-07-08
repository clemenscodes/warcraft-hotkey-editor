mod logic;
mod props;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card::ConflictCard;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
pub use props::UnitPositionConflictCardProps;

/// One position-collision card: the abilities that land on the same command-card
/// cell, flanking (or stacked under) a mini grid flagging that cell. Each of the two
/// layouts renders itself away when it does not apply.
use tw_macro::assert_component;
assert_component!(UnitPositionConflictCard);
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        ConflictCard {
            ConflictCardCaption { ..model.caption }
            ConflictPairRow { ..model.pair_row }
            ConflictMultiStack { ..model.multi_stack }
        }
    }
}
