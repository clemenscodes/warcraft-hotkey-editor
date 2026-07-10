use super::view::InlineHotkeyMarkerView;
use dioxus::prelude::*;

/// The shared-hotkey badge shown inline between two abilities.
#[derive(Props, Clone, PartialEq)]
pub struct InlineHotkeyMarkerProps {
    #[props(into)]
    pub label: String,
}

impl From<&InlineHotkeyMarkerView> for InlineHotkeyMarkerProps {
    fn from(view: &InlineHotkeyMarkerView) -> Self {
        let InlineHotkeyMarkerView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for InlineHotkeyMarkerProps {
    type View = InlineHotkeyMarkerView;
}
