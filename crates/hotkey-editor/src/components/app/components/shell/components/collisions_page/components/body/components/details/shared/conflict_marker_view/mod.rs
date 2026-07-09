pub mod components;
mod props;
mod state;

use components::conflict_position::{ConflictPosition, ConflictPositionProps};
use components::inline_hotkey_marker::{InlineHotkeyMarker, InlineHotkeyMarkerProps};
use components::top_hotkey_marker::{TopHotkeyMarker, TopHotkeyMarkerProps};
use dioxus::prelude::*;
pub use props::ConflictMarkerViewProps;
pub use state::ConflictMarker;

/// The marker shown between (or above) a conflict's abilities: the shared-hotkey badge
/// for a hotkey clash, or the colliding command-card cell for a position clash. A
/// switch on the marker (and, for the hotkey badge, whether it caps a stack) so the pair
/// row and multi stack stay flat lists of children; there is no `data-top` attribute.
use tw_macro::assert_component;
assert_component!(ConflictMarkerView);
#[component]
pub fn ConflictMarkerView(props: ConflictMarkerViewProps) -> Element {
    let is_top = props.is_top;
    match props.marker {
        ConflictMarker::Hotkey { label } => match is_top {
            true => {
                let marker = TopHotkeyMarkerProps { label };
                rsx! {
                    TopHotkeyMarker { ..marker }
                }
            }
            false => {
                let marker = InlineHotkeyMarkerProps { label };
                rsx! {
                    InlineHotkeyMarker { ..marker }
                }
            }
        },
        ConflictMarker::Position { coordinate } => {
            let position = ConflictPositionProps { coordinate, is_top };
            rsx! {
                ConflictPosition { ..position }
            }
        }
    }
}
