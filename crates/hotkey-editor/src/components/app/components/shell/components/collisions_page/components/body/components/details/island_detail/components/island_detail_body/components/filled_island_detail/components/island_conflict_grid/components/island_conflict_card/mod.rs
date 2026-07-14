pub mod components;
mod model;
pub mod presentation;
mod view;

pub use view::IslandConflictCardView;
mod style;

use components::island_conflict_panel::IslandConflictPanel;
use dioxus::prelude::*;
use model::IslandConflictCardModel;
use presentation::IslandConflictCardPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandConflictCard(props: IslandConflictCardModel) -> Element {
    let model = IslandConflictCardPresentation::from(&props);
    let IslandConflictCardPresentation {
        unit,
        own_ability,
        shared_ability,
    } = model;
    rsx! {
        div {
            class: CLASS,
            IslandConflictPanel {
                unit,
                own_ability,
                shared_ability,
            }
        }
    }
}

assert_component!(IslandConflictCard);
