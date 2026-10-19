#[derive(Clone, PartialEq)]
pub struct ActiveManaValueView {
    pub text: String,
}

impl ddd::Layered for ActiveManaValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveManaValueView {}
