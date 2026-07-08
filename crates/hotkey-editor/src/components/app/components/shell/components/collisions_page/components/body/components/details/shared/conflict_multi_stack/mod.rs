mod props;

use super::conflict_ability::ConflictAbility;
use super::conflict_ability_row::ConflictAbilityRow;
use super::conflict_marker_view::{ConflictMarkerView, ConflictMarkerViewProps};
use dioxus::prelude::*;
pub use props::ConflictMultiStackProps;

/// The conflict marker stacked above every clashing ability; renders nothing when
/// the clash is an exact pair (that layout is the pair row instead).
use tw_macro::assert_component;
assert_component!(ConflictMultiStack);
#[component]
pub fn ConflictMultiStack(props: ConflictMultiStackProps) -> Element {
    let abilities = props.abilities;
    if abilities.is_empty() {
        return rsx! {};
    }
    let marker = ConflictMarkerViewProps {
        marker: props.marker,
        is_top: true,
    };
    rsx! {
        ConflictMarkerView { ..marker }
        ConflictAbilityRow {
            is_multi: true,
            for ability in abilities {
                ConflictAbility { ..ability }
            }
        }
    }
}
