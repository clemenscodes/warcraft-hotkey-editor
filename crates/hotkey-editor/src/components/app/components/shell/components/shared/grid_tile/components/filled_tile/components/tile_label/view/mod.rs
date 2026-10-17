#[derive(Clone, PartialEq)]
pub struct TileLabelView {
    pub text: Option<String>,
}

impl ddd::Layered for TileLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TileLabelView {}
