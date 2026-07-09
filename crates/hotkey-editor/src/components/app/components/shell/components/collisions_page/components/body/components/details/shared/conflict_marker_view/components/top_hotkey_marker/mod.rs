mod props;
mod style;

use super::shared::conflict_hotkey_key::ConflictHotkeyKey;
use dioxus::prelude::*;
pub use props::TopHotkeyMarkerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TopHotkeyMarker);

/// The shared-hotkey badge capping a multi-way stack (nudged down).
#[component]
pub fn TopHotkeyMarker(props: TopHotkeyMarkerProps) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ConflictHotkeyKey { text: label }
        }
    }
}
