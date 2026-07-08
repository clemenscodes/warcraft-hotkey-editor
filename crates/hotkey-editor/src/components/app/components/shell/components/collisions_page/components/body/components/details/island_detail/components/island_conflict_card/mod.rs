pub mod components;
mod logic;
mod props;
mod style;

use components::conflict_separator::ConflictSeparator;
use components::island_conflict_ability::IslandConflictAbility;
use components::island_conflict_unit::IslandConflictUnit;
use dioxus::prelude::*;
use logic::IslandConflictCardModel;
pub use props::IslandConflictCardProps;
use style::{ABILITY_ROW, CONFLICT_CARD, PANEL};
use tw_macro::assert_component;
assert_component!(IslandConflictCard);

/// One conflict card: the affected unit on top, its two clashing abilities flanking
/// the centered separator below. It owns its own card surface directly.
#[component]
pub fn IslandConflictCard(props: IslandConflictCardProps) -> Element {
    let model = IslandConflictCardModel::from(&props);
    rsx! {
        div {
            class: CONFLICT_CARD,
            div {
                class: PANEL,
                IslandConflictUnit { ..model.unit }
                div {
                    class: ABILITY_ROW,
                    "data-multi": false,
                    IslandConflictAbility { ..model.own_ability }
                    ConflictSeparator {}
                    IslandConflictAbility { ..model.shared_ability }
                }
            }
        }
    }
}
