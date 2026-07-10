mod logic;
mod props;
mod view;

pub use view::UnitPositionConflictCardView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_panel::ConflictPanel;
use dioxus::prelude::*;
use props::UnitPositionConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;

/// One position-collision card: the abilities that land on the same command-card cell,
/// flanking (or stacked under) a mini grid flagging that cell. It owns its own card
/// shell directly and wraps the collision panel.
#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        div {
            class: CLASS,
            ConflictPanel { model }
        }
    }
}

assert_component!(UnitPositionConflictCard);
