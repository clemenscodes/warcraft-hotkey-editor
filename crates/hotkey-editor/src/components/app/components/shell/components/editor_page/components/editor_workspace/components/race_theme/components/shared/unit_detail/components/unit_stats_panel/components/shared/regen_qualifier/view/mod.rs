#[derive(Clone, PartialEq)]
pub struct RegenQualifierView {
    pub text: Option<&'static str>,
}

impl ddd::Layered for RegenQualifierView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegenQualifierView {}
