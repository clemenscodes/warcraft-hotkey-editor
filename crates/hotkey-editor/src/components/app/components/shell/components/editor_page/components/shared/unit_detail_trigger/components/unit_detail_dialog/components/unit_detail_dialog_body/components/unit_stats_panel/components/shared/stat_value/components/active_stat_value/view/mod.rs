#[derive(Clone, PartialEq)]
pub struct ActiveStatValueView {
    pub text: String,
}

impl ddd::Layered for ActiveStatValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveStatValueView {}
