#[derive(Clone, PartialEq)]
pub struct AltStateLineView {
    pub text: String,
}

impl ddd::View for AltStateLineView {}
