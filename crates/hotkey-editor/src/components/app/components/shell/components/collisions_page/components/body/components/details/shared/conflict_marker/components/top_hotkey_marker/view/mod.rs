#[derive(Clone, PartialEq)]
pub struct TopHotkeyMarkerView {
    pub label: String,
}

impl ddd::Layered for TopHotkeyMarkerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TopHotkeyMarkerView {}
