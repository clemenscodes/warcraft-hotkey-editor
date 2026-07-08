mod props;

use super::conflict_ability::ConflictAbility;
use super::conflict_ability_row::ConflictAbilityRow;
use super::conflict_marker_view::{ConflictMarkerView, ConflictMarkerViewProps};
use dioxus::prelude::*;
pub use props::{AbilityPair, ConflictPairRowProps};

/// The two abilities flanking the conflict marker; renders nothing unless the clash
/// is an exact pair. The marker is the shared hotkey key or the colliding cell.
use tw_macro::assert_component;
assert_component!(ConflictPairRow);
#[component]
pub fn ConflictPairRow(props: ConflictPairRowProps) -> Element {
    let Some(pair) = props.pair else {
        return rsx! {};
    };
    let left = pair.left;
    let right = pair.right;
    let marker = ConflictMarkerViewProps {
        marker: pair.marker,
        is_top: false,
    };
    rsx! {
        ConflictAbilityRow {
            ConflictAbility { ..left }
            ConflictMarkerView { ..marker }
            ConflictAbility { ..right }
        }
    }
}
