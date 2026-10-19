#[derive(Clone, PartialEq)]
pub struct LinkMoveNameView {
    pub text: String,
}

impl ddd::Layered for LinkMoveNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for LinkMoveNameView {}
