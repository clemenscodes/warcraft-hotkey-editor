#[derive(Clone, PartialEq)]
pub struct CarrierCardNameView {
    pub text: String,
}

impl ddd::Layered for CarrierCardNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarrierCardNameView {}
