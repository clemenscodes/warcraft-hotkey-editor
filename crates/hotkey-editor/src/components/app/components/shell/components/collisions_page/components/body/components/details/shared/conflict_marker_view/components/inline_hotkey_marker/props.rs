use dioxus::prelude::*;

/// The shared-hotkey badge shown inline between two abilities.
#[derive(Props, Clone, PartialEq)]
pub struct InlineHotkeyMarkerProps {
    #[props(into)]
    pub label: String,
}
