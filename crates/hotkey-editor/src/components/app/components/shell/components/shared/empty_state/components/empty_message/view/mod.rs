#[derive(Clone, PartialEq)]
pub struct EmptyMessageView {
    pub text: String,
}

impl ddd::Layered for EmptyMessageView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyMessageView {}
