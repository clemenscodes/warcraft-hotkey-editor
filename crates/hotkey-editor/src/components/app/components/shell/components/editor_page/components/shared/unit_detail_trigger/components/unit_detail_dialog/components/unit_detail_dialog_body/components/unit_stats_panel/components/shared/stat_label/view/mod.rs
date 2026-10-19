#[derive(Clone, PartialEq)]
pub struct StatLabelView {
    pub text: String,
}

impl ddd::Layered for StatLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StatLabelView {}
