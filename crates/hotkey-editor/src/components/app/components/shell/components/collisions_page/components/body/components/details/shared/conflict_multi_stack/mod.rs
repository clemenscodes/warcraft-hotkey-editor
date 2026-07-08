mod props;
mod style;

use super::conflict_ability::ConflictAbility;
use super::conflict_marker_view::{ConflictMarkerView, ConflictMarkerViewProps};
use dioxus::prelude::*;
pub use props::ConflictMultiStackProps;
use style::ABILITY_ROW;
use tw_macro::assert_component;
assert_component!(ConflictMultiStack);

/// The stacked layout for a shared-key clash carried by three or more abilities: the
/// key badge over a wrapping row of ability icons. It owns its own abilities row.
/// Renders nothing when there are no abilities to stack.
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
        div {
            class: ABILITY_ROW,
            "data-multi": true,
            for ability in abilities {
                ConflictAbility { ..ability }
            }
        }
    }
}
