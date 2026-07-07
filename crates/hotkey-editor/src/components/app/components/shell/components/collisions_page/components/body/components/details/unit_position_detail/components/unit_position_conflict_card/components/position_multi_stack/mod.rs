mod props;

use super::shared::conflict_position::ConflictPosition;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbility;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability_row::ConflictAbilityRow;
use dioxus::prelude::*;
pub use props::PositionMultiStackProps;

/// The colliding cell stacked above every landing ability; renders nothing when the
/// clash is an exact pair (that layout is the pair row instead).
use tw_macro::assert_component;
assert_component!(PositionMultiStack);
#[component]
pub fn PositionMultiStack(props: PositionMultiStackProps) -> Element {
    let abilities = props.abilities;
    if abilities.is_empty() {
        return rsx! {};
    }
    let cell = props.cell;
    rsx! {
        ConflictPosition { ..cell }
        ConflictAbilityRow {
            is_multi: true,
            for ability in abilities {
                ConflictAbility { ..ability }
            }
        }
    }
}
