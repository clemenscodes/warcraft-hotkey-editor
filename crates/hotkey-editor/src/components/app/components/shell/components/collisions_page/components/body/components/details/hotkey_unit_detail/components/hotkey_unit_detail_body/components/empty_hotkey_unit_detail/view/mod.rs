#[derive(Clone, PartialEq)]
pub struct EmptyHotkeyUnitDetailView {
    pub prompt: String,
}

impl ddd::Layered for EmptyHotkeyUnitDetailView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyHotkeyUnitDetailView {}
