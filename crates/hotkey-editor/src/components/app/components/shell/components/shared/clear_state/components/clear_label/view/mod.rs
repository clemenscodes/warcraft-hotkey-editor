#[derive(Clone, PartialEq)]
pub struct ClearLabelView {
    pub text: String,
}

impl ddd::Layered for ClearLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ClearLabelView {}
