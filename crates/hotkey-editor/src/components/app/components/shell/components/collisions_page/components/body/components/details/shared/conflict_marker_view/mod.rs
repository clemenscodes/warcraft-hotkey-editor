pub mod components;
mod props;
mod state;

use components::conflict_position::ConflictPosition;
use components::inline_hotkey_marker::InlineHotkeyMarker;
use components::top_hotkey_marker::TopHotkeyMarker;
use dioxus::prelude::*;
use props::ConflictMarkerViewProps;
pub use state::ConflictMarker;
use tw_macro::assert_component;

/// The marker shown between (or above) a conflict's abilities: the shared-hotkey badge
/// for a hotkey clash, or the colliding command-card cell for a position clash. A
/// switch on the marker (and, for the hotkey badge, whether it caps a stack) so the pair
/// row and multi stack stay flat lists of children; there is no `data-top` attribute.
#[component]
pub fn ConflictMarkerView(props: ConflictMarkerViewProps) -> Element {
    let is_top = props.is_top;
    match props.marker {
        ConflictMarker::Hotkey { label } => match is_top {
            true => rsx! {
                TopHotkeyMarker { label }
            },
            false => rsx! {
                InlineHotkeyMarker { label }
            },
        },
        ConflictMarker::Position { coordinate } => rsx! {
            ConflictPosition {
                coordinate,
                is_top,
            }
        },
    }
}

assert_component!(ConflictMarkerView);
