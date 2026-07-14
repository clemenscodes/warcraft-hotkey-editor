mod model;
mod view;

pub use view::InlineHotkeyMarkerView;
mod style;

use super::shared::conflict_hotkey_key::ConflictHotkeyKey;
use dioxus::prelude::*;
use model::InlineHotkeyMarkerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InlineHotkeyMarker(props: InlineHotkeyMarkerModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ConflictHotkeyKey {
                text: label,
            }
        }
    }
}

assert_component!(InlineHotkeyMarker);
