#[derive(Clone, PartialEq)]
pub struct EmptyMessageView {
    pub text: String,
}

impl ddd::View for EmptyMessageView {}
