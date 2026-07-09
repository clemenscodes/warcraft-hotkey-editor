mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_panel::{
    ConflictPanel, ConflictPanelProps,
};
use dioxus::prelude::*;
pub use props::UnitPositionConflictCardProps;
use style::CONFLICT_CARD;
use tw_macro::assert_component;
assert_component!(UnitPositionConflictCard);

/// One position-collision card: the abilities that land on the same command-card cell,
/// flanking (or stacked under) a mini grid flagging that cell. It owns its own card
/// shell directly and wraps the collision panel.
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    let panel = ConflictPanelProps {
        caption: model.caption,
        pair_row: model.pair_row,
        multi_stack: model.multi_stack,
    };
    rsx! {
        div {
            class: CONFLICT_CARD,
            ConflictPanel { ..panel }
        }
    }
}
