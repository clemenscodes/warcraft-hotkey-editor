#[derive(Clone, PartialEq)]
pub struct ControlPlainIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::Layered for ControlPlainIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ControlPlainIconView {}
