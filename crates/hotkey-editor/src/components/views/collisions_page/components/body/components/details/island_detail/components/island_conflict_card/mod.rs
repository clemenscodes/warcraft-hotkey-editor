mod logic;
mod props;

use super::conflict_separator::ConflictSeparator;
use super::island_conflict_ability::IslandConflictAbility;
use super::island_conflict_unit::IslandConflictUnit;
use crate::components::views::collisions_page::components::body::components::details::shared::conflict_ability_row::ConflictAbilityRow;
use crate::components::views::collisions_page::components::body::components::details::shared::conflict_card::ConflictCard;
use dioxus::prelude::*;
use logic::IslandConflictCardModel;
pub use props::IslandConflictCardProps;

/// One conflict card: the affected unit on top, its two clashing abilities flanking
/// the centered ✕ below.
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
