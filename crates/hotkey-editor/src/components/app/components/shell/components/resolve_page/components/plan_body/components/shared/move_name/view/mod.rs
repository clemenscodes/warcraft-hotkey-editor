#[derive(Clone, PartialEq)]
pub struct MoveNameView {
    pub text: String,
    pub is_link: bool,
}

impl ddd::Layered for MoveNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MoveNameView {}
