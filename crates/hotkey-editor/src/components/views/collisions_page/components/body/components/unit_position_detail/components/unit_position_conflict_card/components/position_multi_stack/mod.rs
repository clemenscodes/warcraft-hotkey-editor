mod props;

use super::conflict_position_cell::ConflictPositionCell;
use crate::components::views::collisions_page::components::body::components::conflict_ability::ConflictAbility;
use crate::components::views::collisions_page::components::body::components::conflict_ability_row::ConflictAbilityRow;
use dioxus::prelude::*;
pub use props::PositionMultiStackProps;

/// The colliding cell stacked above every landing ability; renders nothing when the
/// clash is an exact pair (that layout is the pair row instead).
#[component]
pub fn PositionMultiStack(props: PositionMultiStackProps) -> Element {
    let abilities = props.abilities;
    if abilities.is_empty() {
        return rsx! {};
    }
    let cell = props.cell;
    rsx! {
        ConflictPositionCell { ..cell }
        ConflictAbilityRow {
            is_multi: true,
            for ability in abilities {
                ConflictAbility { ..ability }
            }
        }
    }
}
