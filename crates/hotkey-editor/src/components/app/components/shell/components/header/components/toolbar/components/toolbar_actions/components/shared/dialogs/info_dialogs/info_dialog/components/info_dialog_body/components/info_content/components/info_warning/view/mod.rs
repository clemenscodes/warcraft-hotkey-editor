#[derive(Clone, PartialEq)]
pub struct InfoWarningView {
    pub warning: Option<&'static str>,
}

impl ddd::Layered for InfoWarningView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InfoWarningView {}
