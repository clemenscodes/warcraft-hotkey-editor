#[derive(Clone, PartialEq)]
pub struct AltStateLabelView {
    pub text: Option<String>,
}

impl ddd::Layered for AltStateLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AltStateLabelView {}
