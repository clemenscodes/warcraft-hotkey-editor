#[derive(Clone, PartialEq)]
pub struct RegenLabelView {
    pub text: String,
}

impl ddd::Layered for RegenLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegenLabelView {}
