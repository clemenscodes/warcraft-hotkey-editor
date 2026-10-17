#[derive(Clone, PartialEq)]
pub struct UnbindableNoteView {
    pub text: String,
}

impl ddd::Layered for UnbindableNoteView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnbindableNoteView {}
