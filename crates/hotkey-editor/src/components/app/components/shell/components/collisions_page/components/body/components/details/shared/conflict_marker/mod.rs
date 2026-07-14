pub mod components;
mod model;
mod view;

pub use view::ConflictMarkerView;
mod state;

use components::conflict_position::ConflictPosition;
use components::inline_hotkey_marker::InlineHotkeyMarker;
use components::top_hotkey_marker::TopHotkeyMarker;
use dioxus::prelude::*;
use model::ConflictMarkerModel;
pub use state::ConflictMarker;
use tw_macro::assert_component;

#[component]
pub fn ConflictMarker(props: ConflictMarkerModel) -> Element {
    let is_top = props.is_top;
    match props.marker {
        ConflictMarker::Hotkey { label } => match is_top {
            true => rsx! {
                TopHotkeyMarker {
                    label,
                }
            },
            false => rsx! {
                InlineHotkeyMarker {
                    label,
                }
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

assert_component!(ConflictMarker);
