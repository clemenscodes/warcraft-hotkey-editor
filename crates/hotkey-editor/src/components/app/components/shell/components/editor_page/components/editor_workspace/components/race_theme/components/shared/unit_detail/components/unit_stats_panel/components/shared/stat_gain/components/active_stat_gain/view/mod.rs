#[derive(Clone, PartialEq)]
pub struct ActiveStatGainView {
    pub text: String,
}

impl ddd::Layered for ActiveStatGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveStatGainView {}
