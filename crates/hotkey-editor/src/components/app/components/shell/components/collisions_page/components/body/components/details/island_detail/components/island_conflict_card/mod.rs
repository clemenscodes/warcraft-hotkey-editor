pub mod components;
mod logic;
mod props;

use components::conflict_separator::ConflictSeparator;
use components::island_conflict_ability::IslandConflictAbility;
use components::island_conflict_unit::IslandConflictUnit;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_row::ConflictAbilityRow;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card::ConflictCard;
use dioxus::prelude::*;
use logic::IslandConflictCardModel;
pub use props::IslandConflictCardProps;

/// One conflict card: the affected unit on top, its two clashing abilities flanking
/// the centered ✕ below.
use tw_macro::assert_component;
assert_component!(IslandConflictCard);
#[component]
pub fn IslandConflictCard(props: IslandConflictCardProps) -> Element {
    let model = IslandConflictCardModel::from(&props);
    rsx! {
        ConflictCard {
            IslandConflictUnit { ..model.unit }
            ConflictAbilityRow {
                IslandConflictAbility { ..model.own_ability }
                ConflictSeparator {}
                IslandConflictAbility { ..model.shared_ability }
            }
        }
    }
}
