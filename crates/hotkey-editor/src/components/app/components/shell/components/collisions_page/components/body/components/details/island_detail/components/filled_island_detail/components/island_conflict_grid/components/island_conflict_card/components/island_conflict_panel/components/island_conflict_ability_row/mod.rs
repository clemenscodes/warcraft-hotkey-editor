pub mod components;
mod props;
mod style;

use components::conflict_separator::ConflictSeparator;
use components::island_conflict_ability::IslandConflictAbility;
use dioxus::prelude::*;
pub use props::IslandConflictAbilityRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The two clashing abilities flanking the centered separator. It owns its own abilities
/// row element directly.
#[component]
pub fn IslandConflictAbilityRow(props: IslandConflictAbilityRowProps) -> Element {
    let own_ability = props.own_ability;
    let shared_ability = props.shared_ability;
    rsx! {
        div {
            class: CLASS,
            IslandConflictAbility { ..own_ability }
            ConflictSeparator {}
            IslandConflictAbility { ..shared_ability }
        }
    }
}

assert_component!(IslandConflictAbilityRow);
