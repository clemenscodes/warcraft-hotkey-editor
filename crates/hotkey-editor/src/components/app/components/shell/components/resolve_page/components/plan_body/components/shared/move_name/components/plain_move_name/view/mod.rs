#[derive(Clone, PartialEq)]
pub struct PlainMoveNameView {
    pub text: String,
}

impl ddd::View for PlainMoveNameView {}
