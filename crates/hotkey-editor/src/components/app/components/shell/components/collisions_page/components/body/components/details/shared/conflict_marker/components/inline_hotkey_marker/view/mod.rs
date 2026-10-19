#[derive(Clone, PartialEq)]
pub struct InlineHotkeyMarkerView {
    pub label: String,
}

impl ddd::Layered for InlineHotkeyMarkerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InlineHotkeyMarkerView {}
