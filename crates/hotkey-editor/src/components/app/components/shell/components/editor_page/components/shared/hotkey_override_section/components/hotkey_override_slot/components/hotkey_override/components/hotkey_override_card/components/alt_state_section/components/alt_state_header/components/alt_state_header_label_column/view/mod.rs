#[derive(Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnView {
    pub text: Option<String>,
}

impl ddd::Layered for AltStateHeaderLabelColumnView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AltStateHeaderLabelColumnView {}
