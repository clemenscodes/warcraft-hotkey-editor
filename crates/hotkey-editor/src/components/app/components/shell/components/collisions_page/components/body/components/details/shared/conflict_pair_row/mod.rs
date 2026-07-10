mod props;
mod style;

use super::conflict_ability::ConflictAbility;
use super::conflict_marker_view::{ConflictMarkerView, ConflictMarkerViewProps};
use dioxus::prelude::*;
pub use props::{AbilityPair, ConflictPairRowProps};
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictPairRow);

/// The two-ability layout for a shared-key clash: the two abilities flanking the key
/// badge. It owns its own abilities row. Renders nothing when there is no pair.
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
        div {
            class: CLASS,
            ConflictAbility { ..left }
            ConflictMarkerView { ..marker }
            ConflictAbility { ..right }
        }
    }
}
