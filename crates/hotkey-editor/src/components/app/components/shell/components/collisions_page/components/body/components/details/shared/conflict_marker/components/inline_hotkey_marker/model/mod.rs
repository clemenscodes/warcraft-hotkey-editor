use super::view::InlineHotkeyMarkerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InlineHotkeyMarkerModel {
    #[props(into)]
    pub label: String,
}

impl From<&InlineHotkeyMarkerView> for InlineHotkeyMarkerModel {
    fn from(view: &InlineHotkeyMarkerView) -> Self {
        let InlineHotkeyMarkerView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for InlineHotkeyMarkerModel {
    type View = InlineHotkeyMarkerView;
}
