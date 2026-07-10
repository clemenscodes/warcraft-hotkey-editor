mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_panel::{
    ConflictPanel, ConflictPanelProps,
};
use dioxus::prelude::*;
pub use props::UnitPositionConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitPositionConflictCard);

/// One position-collision card: the abilities that land on the same command-card cell,
/// flanking (or stacked under) a mini grid flagging that cell. It owns its own card
/// shell directly and wraps the collision panel.
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    let panel = ConflictPanelProps::from(model);
    rsx! {
        div {
            class: CLASS,
            ConflictPanel { ..panel }
        }
    }
}
