pub mod components;
mod props;
mod style;

use super::super::super::super::logic::IslandAbilityData;
use components::conflict_separator::ConflictSeparator;
use components::island_conflict_ability::IslandConflictAbility;
use dioxus::prelude::*;
use props::IslandConflictAbilityRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The two clashing abilities flanking the centered separator. It owns its own abilities
/// row element directly.
#[component]
pub fn IslandConflictAbilityRow(props: IslandConflictAbilityRowProps) -> Element {
    let IslandConflictAbilityRowProps {
        own_ability,
        shared_ability,
    } = props;
    let IslandAbilityData {
        ability_name: own_name,
        ability_id: own_id,
        icon_url: own_icon,
        extra_count: own_extra,
        inspected: own_inspected,
    } = own_ability;
    let IslandAbilityData {
        ability_name: shared_name,
        ability_id: shared_id,
        icon_url: shared_icon,
        extra_count: shared_extra,
        inspected: shared_inspected,
    } = shared_ability;
    rsx! {
        div {
            class: CLASS,
            IslandConflictAbility {
                ability_name: own_name,
                ability_id: own_id,
                icon_url: own_icon,
                extra_count: own_extra,
                inspected: own_inspected,
            }
            ConflictSeparator {}
            IslandConflictAbility {
                ability_name: shared_name,
                ability_id: shared_id,
                icon_url: shared_icon,
                extra_count: shared_extra,
                inspected: shared_inspected,
            }
        }
    }
}

assert_component!(IslandConflictAbilityRow);
