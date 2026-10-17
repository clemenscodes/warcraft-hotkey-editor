#[derive(Clone, PartialEq)]
pub struct PrimaryAttributeLabelView {
    pub text: String,
}

impl ddd::Layered for PrimaryAttributeLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PrimaryAttributeLabelView {}
