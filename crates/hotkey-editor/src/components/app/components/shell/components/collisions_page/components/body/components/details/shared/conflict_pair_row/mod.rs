mod model;
mod view;

pub use view::ConflictPairRowView;
mod state;
mod style;

use super::conflict_ability::ConflictAbility;
use super::conflict_card_model::ConflictAbilityData;
use super::conflict_marker::ConflictMarker;
use dioxus::prelude::*;
use model::ConflictPairRowModel;
pub use state::AbilityPair;
use style::CLASS;
use tw_macro::assert_component;

/// The two-ability layout for a shared-key clash: the two abilities flanking the key
/// badge. It owns its own abilities row. Renders nothing when there is no pair.
#[component]
pub fn ConflictPairRow(props: ConflictPairRowModel) -> Element {
    let Some(pair) = props.pair else {
        return rsx! {};
    };
    let AbilityPair {
        left,
        right,
        marker,
    } = pair;
    let ConflictAbilityData {
        name: left_name,
        ability_id: left_ability_id,
        icon_url: left_icon_url,
        unit_id: left_unit_id,
    } = left;
    let ConflictAbilityData {
        name: right_name,
        ability_id: right_ability_id,
        icon_url: right_icon_url,
        unit_id: right_unit_id,
    } = right;
    rsx! {
        div {
            class: CLASS,
            ConflictAbility {
                ability_name: left_name,
                ability_id: left_ability_id,
                icon_url: left_icon_url,
                unit_id: left_unit_id,
            }
            ConflictMarker {
                marker,
                is_top: false,
            }
            ConflictAbility {
                ability_name: right_name,
                ability_id: right_ability_id,
                icon_url: right_icon_url,
                unit_id: right_unit_id,
            }
        }
    }
}

assert_component!(ConflictPairRow);
