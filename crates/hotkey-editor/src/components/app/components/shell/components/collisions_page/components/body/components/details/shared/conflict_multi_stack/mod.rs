mod props;
mod view;

pub use view::ConflictMultiStackView;
mod style;

use super::conflict_ability::ConflictAbility;
use super::conflict_card_model::ConflictAbilityData;
use super::conflict_marker_view::ConflictMarkerView;
use dioxus::prelude::*;
use props::ConflictMultiStackProps;
use style::CLASS;
use tw_macro::assert_component;

/// The stacked layout for a shared-key clash carried by three or more abilities: the
/// key badge over a wrapping row of ability icons. It owns its own abilities row.
/// Renders nothing when there are no abilities to stack.
#[component]
pub fn ConflictMultiStack(props: ConflictMultiStackProps) -> Element {
    let abilities = props.abilities;
    if abilities.is_empty() {
        return rsx! {};
    }
    let marker = props.marker;
    rsx! {
        ConflictMarkerView {
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
