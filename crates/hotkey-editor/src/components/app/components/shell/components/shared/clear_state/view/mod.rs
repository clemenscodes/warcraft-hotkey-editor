#[derive(Clone, PartialEq)]
pub struct ClearStateView {
    pub text: &'static str,
}

impl ddd::Layered for ClearStateView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ClearStateView {}
