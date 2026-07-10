mod props;
mod style;

use super::shared::conflict_hotkey_key::ConflictHotkeyKey;
use dioxus::prelude::*;
pub use props::InlineHotkeyMarkerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The shared-hotkey badge shown inline between two abilities.
#[component]
pub fn InlineHotkeyMarker(props: InlineHotkeyMarkerProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ConflictHotkeyKey { text: label }
        }
    }
}

assert_component!(InlineHotkeyMarker);
