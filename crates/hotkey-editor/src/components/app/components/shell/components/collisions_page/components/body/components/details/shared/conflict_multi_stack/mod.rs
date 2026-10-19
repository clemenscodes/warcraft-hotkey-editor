mod model;
mod view;

pub use view::ConflictMultiStackView;
mod style;

use super::conflict_ability::ConflictAbility;
use super::conflict_card_model::ConflictAbilityData;
use super::conflict_marker::ConflictMarker;
use dioxus::prelude::*;
use model::ConflictMultiStackModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictMultiStack(props: ConflictMultiStackModel) -> Element {
    let abilities = props.abilities;
    if abilities.is_empty() {
        return rsx! {};
    }
    let marker = props.marker;
    rsx! {
        ConflictMarker {
            marker,
            is_top: true,
        }
        div {
            class: CLASS,
            for ConflictAbilityData { name, ability_id, icon_url, unit_id } in abilities {
                ConflictAbility {
                    ability_name: name,
                    ability_id,
                    icon_url,
                    unit_id,
                }
            }
        }
    }
}

assert_component!(ConflictMultiStack);
