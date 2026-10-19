#[derive(Clone, PartialEq)]
pub struct ConflictCardCaptionView {
    pub text: String,
}

impl ddd::Layered for ConflictCardCaptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictCardCaptionView {}
