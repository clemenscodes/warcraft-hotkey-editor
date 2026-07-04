mod props;

use super::conflict_position_cell::ConflictPositionCell;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbility;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_row::ConflictAbilityRow;
use dioxus::prelude::*;
pub use props::{PositionPair, PositionPairRowProps};

/// Two abilities flanking the colliding cell; renders nothing unless the clash is
/// an exact pair.
#[component]
pub fn PositionPairRow(props: PositionPairRowProps) -> Element {
    let Some(pair) = props.pair else {
        return rsx! {};
    };
    let left = pair.left;
    let right = pair.right;
    let cell = pair.cell;
    rsx! {
        ConflictAbilityRow {
            ConflictAbility { ..left }
            ConflictPositionCell { ..cell }
            ConflictAbility { ..right }
        }
    }
}
