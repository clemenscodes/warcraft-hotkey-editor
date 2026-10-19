#[derive(Clone, PartialEq)]
pub struct ActiveManaRegenGainView {
    pub text: String,
}

impl ddd::Layered for ActiveManaRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveManaRegenGainView {}
