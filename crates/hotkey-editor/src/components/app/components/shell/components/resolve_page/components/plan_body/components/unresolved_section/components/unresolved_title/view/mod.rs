#[derive(Clone, PartialEq)]
pub struct UnresolvedTitleView {
    pub text: &'static str,
}

impl ddd::Layered for UnresolvedTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnresolvedTitleView {}
