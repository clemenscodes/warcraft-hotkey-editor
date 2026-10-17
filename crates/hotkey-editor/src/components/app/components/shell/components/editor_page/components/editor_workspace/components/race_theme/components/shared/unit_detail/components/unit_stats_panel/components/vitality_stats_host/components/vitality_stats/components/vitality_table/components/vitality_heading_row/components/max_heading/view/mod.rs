#[derive(Clone, PartialEq)]
pub struct MaxHeadingView {
    pub text: &'static str,
}

impl ddd::Layered for MaxHeadingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MaxHeadingView {}
