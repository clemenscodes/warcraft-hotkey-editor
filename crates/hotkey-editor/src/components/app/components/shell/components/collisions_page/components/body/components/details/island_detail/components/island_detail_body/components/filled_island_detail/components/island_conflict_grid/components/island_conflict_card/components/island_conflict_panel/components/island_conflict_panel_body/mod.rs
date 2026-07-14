pub mod components;
mod model;
mod style;
mod view;

pub use model::IslandConflictCardData;
pub use view::IslandConflictPanelBodyView;

use super::super::super::super::presentation::IslandUnitData;
use components::island_conflict_ability_row::IslandConflictAbilityRow;
use components::island_conflict_unit::IslandConflictUnit;
use dioxus::prelude::*;
use model::IslandConflictPanelBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandConflictPanelBody(props: IslandConflictPanelBodyModel) -> Element {
    let cards = props.cards;
    if let Some(card) = cards.into_iter().next() {
        let IslandConflictCardData {
            unit,
            own_ability,
            shared_ability,
        } = card;
        let IslandUnitData {
            unit_id,
            icon_url,
            name,
        } = unit;
        rsx! {
            div {
                class: CLASS,
                IslandConflictUnit {
                    unit_id,
                    icon_url,
                    name,
                }
                IslandConflictAbilityRow {
                    own_ability,
                    shared_ability,
                }
            }
        }
    } else {
        rsx! {
            div {
                class: CLASS,
            }
        }
    }
}

assert_component!(IslandConflictPanelBody);
