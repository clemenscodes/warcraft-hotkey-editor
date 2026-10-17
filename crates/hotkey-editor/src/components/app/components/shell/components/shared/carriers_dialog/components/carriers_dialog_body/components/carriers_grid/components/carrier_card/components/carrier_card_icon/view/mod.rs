#[derive(Clone, PartialEq)]
pub struct CarrierCardIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for CarrierCardIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CarrierCardIconView {}
