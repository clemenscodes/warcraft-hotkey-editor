mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
pub use props::UnitPositionConflictCardProps;
use style::{CONFLICT_CARD, PANEL};
use tw_macro::assert_component;
assert_component!(UnitPositionConflictCard);

/// One position-collision card: the abilities that land on the same command-card cell,
/// flanking (or stacked under) a mini grid flagging that cell. It owns its own card
/// surface directly. Each of the two layouts renders itself away when it does not
/// apply.
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        div {
            class: CONFLICT_CARD,
            div {
                class: PANEL,
                ConflictCardCaption { ..model.caption }
                ConflictPairRow { ..model.pair_row }
                ConflictMultiStack { ..model.multi_stack }
            }
        }
    }
}
