#[derive(Clone, PartialEq)]
pub struct UnbindableNoteView {
    pub text: String,
}

impl ddd::View for UnbindableNoteView {}
