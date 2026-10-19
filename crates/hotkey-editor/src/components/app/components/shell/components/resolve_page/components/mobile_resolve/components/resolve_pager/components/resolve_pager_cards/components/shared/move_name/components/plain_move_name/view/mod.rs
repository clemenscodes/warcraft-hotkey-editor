#[derive(Clone, PartialEq)]
pub struct PlainMoveNameView {
    pub text: String,
}

impl ddd::Layered for PlainMoveNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PlainMoveNameView {}
