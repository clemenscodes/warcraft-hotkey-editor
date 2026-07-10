use super::view::TopHotkeyMarkerView;
use dioxus::prelude::*;

/// The shared-hotkey badge capping a multi-way stack (nudged down).
#[derive(Props, Clone, PartialEq)]
pub struct TopHotkeyMarkerProps {
    #[props(into)]
    pub label: String,
}

impl From<&TopHotkeyMarkerView> for TopHotkeyMarkerProps {
    fn from(view: &TopHotkeyMarkerView) -> Self {
        let TopHotkeyMarkerView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for TopHotkeyMarkerProps {
    type View = TopHotkeyMarkerView;
}
