mod model;
mod presentation;
mod view;

pub use view::UnitPositionConflictCardView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_panel::ConflictPanel;
use dioxus::prelude::*;
use model::UnitPositionConflictCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitPositionConflictCard(props: UnitPositionConflictCardModel) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        div {
            class: CLASS,
            ConflictPanel {
                model,
            }
        }
    }
}

assert_component!(UnitPositionConflictCard);
