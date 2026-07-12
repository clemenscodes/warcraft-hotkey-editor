use super::view::TopHotkeyMarkerView;
use dioxus::prelude::*;

/// The shared-hotkey badge capping a multi-way stack (nudged down).
#[derive(Props, Clone, PartialEq)]
pub struct TopHotkeyMarkerModel {
    #[props(into)]
    pub label: String,
}

impl From<&TopHotkeyMarkerView> for TopHotkeyMarkerModel {
    fn from(view: &TopHotkeyMarkerView) -> Self {
        let TopHotkeyMarkerView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for TopHotkeyMarkerModel {
    type View = TopHotkeyMarkerView;
}
