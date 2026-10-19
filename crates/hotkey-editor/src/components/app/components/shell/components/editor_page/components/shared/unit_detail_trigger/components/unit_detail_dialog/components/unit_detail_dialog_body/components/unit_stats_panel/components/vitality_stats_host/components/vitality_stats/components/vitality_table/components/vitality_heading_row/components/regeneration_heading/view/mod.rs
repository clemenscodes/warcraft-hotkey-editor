#[derive(Clone, PartialEq)]
pub struct RegenerationHeadingView {
    pub text: &'static str,
}

impl ddd::Layered for RegenerationHeadingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegenerationHeadingView {}
