pub mod components;
mod props;
mod state;

use components::conflict_hotkey_badge::ConflictHotkeyBadge;
use components::conflict_hotkey_key::ConflictHotkeyKey;
use components::conflict_position::{ConflictPosition, ConflictPositionProps};
use dioxus::prelude::*;
pub use props::ConflictMarkerViewProps;
pub use state::ConflictMarker;

/// The marker shown between (or above) a conflict's abilities: the shared-hotkey
/// badge for a hotkey clash, or the colliding command-card cell for a position
/// clash. A dedicated switch on the marker, so the pair row and multi stack stay
/// flat lists of children.
use tw_macro::assert_component;
assert_component!(ConflictMarkerView);
#[component]
pub fn ConflictMarkerView(props: ConflictMarkerViewProps) -> Element {
    let is_top = props.is_top;
    match props.marker {
        ConflictMarker::Hotkey { label } => {
            rsx! {
                ConflictHotkeyBadge {
                    is_top,
                    ConflictHotkeyKey { text: label }
                }
            }
        }
        ConflictMarker::Position { coordinate } => {
            let position = ConflictPositionProps { coordinate, is_top };
            rsx! {
                ConflictPosition { ..position }
            }
        }
    }
}
