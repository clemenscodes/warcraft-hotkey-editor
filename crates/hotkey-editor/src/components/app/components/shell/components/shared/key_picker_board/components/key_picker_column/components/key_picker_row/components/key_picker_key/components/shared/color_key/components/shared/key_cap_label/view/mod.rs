#[derive(Clone, PartialEq)]
pub struct KeyCapLabelView {
    pub label: String,
}

impl ddd::Layered for KeyCapLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for KeyCapLabelView {}
