#[derive(Clone, PartialEq)]
pub struct EditingKeycapView {
    pub label: String,
}

impl ddd::Layered for EditingKeycapView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EditingKeycapView {}
